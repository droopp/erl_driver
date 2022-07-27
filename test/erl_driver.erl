-module(erl_driver).
-export([start/0, stop/0, init/1]).
-export([call/1]).

start() ->
    case erl_ddll:load_driver("../", "erl_driver") of
        ok -> ok;
        {error, already_loaded} -> ok;
        _ -> exit({error, could_not_load_driver})
    end,
    spawn(?MODULE, init, ["erl_driver"]).

init(SharedLib) ->
    register(erl_driver, self()),
    Port = open_port({spawn, SharedLib}, [binary]),
    loop(Port).

stop() ->
    erl_driver ! stop.


call(X) ->
    call_port(X).


call_port(Msg) ->
    erl_driver ! {call, self(), Msg},
    receive
        {erl_driver, Result} ->
            Result
    end.


loop(Port) ->
    receive
	{call, Caller, Msg} ->
	    Port ! {self(), {command, Msg}},
	    receive
            {Port, {data, Data}} ->
                Caller ! {erl_driver, Data}
	    end,
	    loop(Port);

	stop ->
	    Port ! {self(), close},
	    receive
            {Port, closed} ->
                exit(normal)
	    end;

	{'EXIT', Port, Reason} ->
	    io:format("~p ~n", [Reason]),
	    exit(port_terminated)
    end.
