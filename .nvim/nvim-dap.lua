local dap = require("dap")

dap.adapters.lldb = {
  type = "executable",
  command = "lldb-dap", -- adjust as needed
  name = "lldb",
}

dap.configurations.rust = {
  {
    name = "Exercise: setup",
    type = "lldb",
    request = "launch",
    program = "target/debug/setup",
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
  },
}
