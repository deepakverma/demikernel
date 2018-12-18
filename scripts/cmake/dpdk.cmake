if(NOT DPDK_DOT_CMAKE_INCLUDED)
set(DPDK_DOT_CMAKE_INCLUDED YES)

include(ExternalProject)
include(list)
include(azure)

option(DPDK_MELLANOX_SUPPORT "Include DPDK support for the Mellanox adaptor" OFF)
set(DPDK_TARGET x86_64-native-linuxapp-gcc CACHE STRING "The DPDK Target")

# DPDK
set(DPDK_SOURCE_DIR ${CMAKE_SOURCE_DIR}/submodules/dpdk)
set(DPDK_BINARY_DIR ${CMAKE_BINARY_DIR}/ExternalProject/dpdk)
set(DPDK_INSTALL_DIR ${DPDK_BINARY_DIR})
set(DPDK_INCLUDE_DIR ${DPDK_INSTALL_DIR}/include ${DPDK_INSTALL_DIR}/include/dpdk)
set(DPDK_LIB_DIR ${DPDK_INSTALL_DIR}/lib)

# we hacked the DPDK build to divulge the flags it generated for
# compilation and linking-- a technique borrowed from mTCP.
set(DPDK_CFLAGS_FILE ${DPDK_SOURCE_DIR}/${DPDK_TARGET}/include/cflags.txt)
set(DPDK_LDFLAGS_FILE ${DPDK_SOURCE_DIR}/${DPDK_TARGET}/lib/ldflags.txt)

if(CMAKE_BUILD_TYPE STREQUAL Debug)
    set(DPDK_DEBUG_FLAG --debug)
endif(CMAKE_BUILD_TYPE STREQUAL Debug)

ExternalProject_Add(dpdk
    PREFIX ${DPDK_BINARY_DIR}
    SOURCE_DIR ${DPDK_SOURCE_DIR}
    CONFIGURE_COMMAND ${CMAKE_SOURCE_DIR}/scripts/build/dpdk.sh -s ${DPDK_SOURCE_DIR} --target ${DPDK_TARGET} ${DPDK_DEBUG_FLAG} --config
    BUILD_COMMAND ${CMAKE_SOURCE_DIR}/scripts/build/dpdk.sh -s ${DPDK_SOURCE_DIR} --target ${DPDK_TARGET} ${DPDK_DEBUG_FLAG}
    INSTALL_COMMAND make -C ${DPDK_SOURCE_DIR} install T=${DPDK_TARGET} DESTDIR=${DPDK_INSTALL_DIR}
)

# configure DPDK options.
if(DPDK_MELLANOX_SUPPORT OR AZURE_SUPPORT)
    set(DPDK_CONFIG_RTE_LIBRTR_MLX4_PMD y)
    set(DPDK_CONFIG_RTE_LIBRTR_MLX5_PMD y)
    if(AZURE_SUPPORT)
        set(DPDK_CONFIG_RTE_LIBRTE_VDEV_NETVSC_PMD y)
    endif(AZURE_SUPPORT)
else(DPDK_MELLANOX_SUPPORT OR AZURE_SUPPORT)
    set(DPDK_CONFIG_RTE_LIBRTR_MLX4_PMD n)
    set(DPDK_CONFIG_RTE_LIBRTR_MLX5_PMD n)
    set(DPDK_CONFIG_RTE_LIBRTE_VDEV_NETVSC_PMD n)
endif(DPDK_MELLANOX_SUPPORT OR AZURE_SUPPORT)
set(DPDK_CONFIG_COMMON_BASE ${DPDK_SOURCE_DIR}/config/common_base)
configure_file(${DPDK_CONFIG_COMMON_BASE}.in ${DPDK_CONFIG_COMMON_BASE})

function(target_add_dpdk TARGET)
    target_include_directories(${TARGET} PUBLIC ${DPDK_INCLUDE_DIR})
    set_target_properties(${TARGET} PROPERTIES
        COMPILE_FLAGS @${DPDK_CFLAGS_FILE}
        LINK_FLAGS @${DPDK_LDFLAGS_FILE}
    )
    add_dependencies(${TARGET} dpdk)
endfunction(target_add_dpdk)

endif(NOT DPDK_DOT_CMAKE_INCLUDED)
