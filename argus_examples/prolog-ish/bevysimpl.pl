%% Encoding of /examples/bevy-more-simplified
%%
%% Removed associated types for simplicity.

world_query(entity).
resource(timer).


% system_param/1
system_param(query(X)) :- world_query(X).
system_param(res(X)) :- resource(X).
system_param([P|Ps]) :-
    system_param(P),
    system_param(Ps),


system_param_function(fn([])).
system_param_function(fn(Args)) :- system_param(Args).


exclusive_system_param_function(fn([world])).


into_system(F) :- system_param_function(F).
into_system(F) :- exclusive_system_param_function(F).


into_system_configs(F) :- into_system(F).


typeck_main :- into_system_configs(fn([timer])).
