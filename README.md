# Diorite DF
A toy language that transpiles to Diamond Fire code blocks.
The syntax can be summed up as
```
pevent BreakBlock
    set = (var(player), "%default")
    set = (var(location), tag<default>(' '))
    paction SendMessage <default> ['Alignment Mode': 'Centered'] ($"<gold>Player", var(player), "broke a block while standing at", location)
end
```

This language does NOT have:
1. Type safety
2. Easy function calls
3. Template strings or any `%whatever()` checks
4. OOP, functional or even any complex data structure
5. Return types and expressions
6. Checks to see:
   1. If a function exists
   2. If a variable exists
7. Easy item creation 
```js
item(`{id:stone,Count:1,tag:{display:{Name:'[{"text":"DumpsterFire","italic":false}]',Lore:['[{"text":"Good luck writing this","italic":false}]']}}}`)
```

`I will finish this`
