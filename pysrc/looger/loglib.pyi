class Logger:
    def __init__(self, logfile: str) -> None:
        """`Create Logger class instance`
        ### Parameters
        - logfile `(str)`: path of the logfile.

        `Any path provided should be valid. It is not required for the logfile to be present.`
        """
    def log(self, content: str, log_to_console: bool) -> None:
        """`log with default log level.`"""
    def info(self, content: str, log_to_console: bool) -> None:
        """`log with log level: INFO`"""
    def warn(self, content: str, log_to_console: bool) -> None:
        """`log with log leve: WARN`"""
    def err(self, content: str, log_to_console: bool) -> None:
        """`log with log level: ERROR`"""
    def debug(self, content: str, log_to_console: bool) -> None:
        """`log with log level: DEBUG`"""