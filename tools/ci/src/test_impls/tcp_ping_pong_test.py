# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

from ci.src.base_test import BaseTest


class TcpPingPongTest(BaseTest):
    test_name = "tcp-ping-pong"

    def execute(self):
        for _, params in self.test_details.items():
            self.run_scenario(params)
        return self.has_test_passed

    def run_scenario(self, params):
        test_alias = self.__make_test_alias(params)
        server_args = self.__make_server_args(params)
        client_args = self.__make_client_args(params)
        s = self.scaffolding
        self.has_test_passed = self.job_test_system_rust(
            test_alias, self.test_name, s["repository"], s["libos"], s["is_debug"], s["server_name"],
            s["client_name"], server_args, client_args, s["is_sudo"], True, s["delay"], s["config_path"],
            s["log_directory"])

    def __make_test_alias(self, params):
        return f"tcp-ping-pong-{params['nrounds']}"

    def __make_server_args(self, params):
        return f"--server {self.scaffolding['server_ip']}:12345 {params['nrounds']} {params['bufsize']}"

    def __make_client_args(self, params):
        # This is deliberately the server_ip
        return f"--client {self.scaffolding['server_ip']}:12345 {params['nrounds']} {params['bufsize']}"
