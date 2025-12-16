package client

CLIState :: enum 
{
    SETUP,
    LOGIN,
    SYNC,
}


State :: struct
{
    connection_status: ConnectionStatus,
    cli_state: CLIState
}
