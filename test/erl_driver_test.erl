-module(erl_driver_test).
-include_lib("eunit/include/eunit.hrl").


exec_call_test_() ->
    {setup,
     fun() ->
             ok
     end,
     fun(_) ->
             ok

     end,
     {
      foreach,
      fun() ->
        erl_driver:start()
        
      end,
      fun(_) ->
        erl_driver:stop()

      end,
      run_tests()

     }
    }.

run_tests() ->
    [
     {"test call",
        fun() ->

            Res = erl_driver:call(<<"test">>),
              ?assert(Res=:=<<"test">>)

        end
     },
     {"test error call",
        fun() ->

            Res = erl_driver:call(<<"error\n">>),
              ?assert(Res=:=<<"error">>)

        end
     },
     {"test call2",
        fun() ->

            Res = erl_driver:call(<<"test">>),
              ?assert(Res=:=<<"test">>)

        end
     }
 

    ].


