### Nyaddec Routes

## host:8000 => PartyServer

# `/api/parties`
get all the parties from the database.

**parameters**
None

**returns**
`DetailedResponse<Vec<PartyEntity>>`
example:
```
{
    data: [
        {
            id: '32bit string as Guid',
            party_name: 'string',
            characters: [
                    {
                        id: '32bit string as Guid',
                        player_id: 'number',
                        player_name: 'string',
                        player_hitpoints: 'number',
                        player_level: 'number',
                        party_id: '32bit string as Guid'
                    },
                    {
                        id: '32bit string as Guid',
                        player_id: 'number',
                        player_name: 'string',
                        player_hitpoints: 'number',
                        player_level: 'number',
                        party_id: '32bit string as Guid'
                    }
                ]
            ]
        }
    ],
    success: 'bool',
    message: 'string'
}
```

# `/api/party/{party_id}`
gets a party using the partys primary key.

**parameters**
`{partyid}: String`
a 32bit string that the api will use to check if there is a party stored with that guid.

**returns**
`DetailedResponse<PartyEntity>`

*SUCESS*
example:
```
{
    data: {
        id: '32bit string as Guid',
        party_name: 'string',
        characters: [
                {
                    id: '32bit string as Guid',
                    player_id: 'number',
                    player_name: 'string',
                    player_hitpoints: 'number',
                    player_level: 'number',
                    party_id: '32bit string as Guid'
                },
                {
                    id: '32bit string as Guid',
                    player_id: 'number',
                    player_name: 'string',
                    player_hitpoints: 'number',
                    player_level: 'number',
                    party_id: '32bit string as Guid'
                }
            ] 
        },
    success: true,
    message: 'Successful api transaction response.'
}
```
*ERROR*
example
```
    data: null,
    success: false,
    message: 'Some error occurred.'
```


# `/api/party/post`

**parameters**

*route: PartyEntity*
a PartyEntity object modeled in json format.

example:
```
{
    id: '32bit string as Guid',
    party_name: 'string',
    characters: [
            {
                id: '32bit string as Guid',
                player_id: 'number',
                player_name: 'string',
                player_hitpoints: 'number',
                player_level: 'number',
                party_id: '32bit string as Guid'
            },
            {
                id: '32bit string as Guid',
                player_id: 'number',
                player_name: 'string',
                player_hitpoints: 'number',
                player_level: 'number',
                party_id: '32bit string as Guid'
            }
        ] 
    }
}
```

**returns**
`DetailedResponse<PartyEntity>`

*SUCCESS*
example:
```
{
    data: {
        id: '32bit string as Guid',
        party_name: 'string',
        characters: [
                {
                    id: '32bit string as Guid',
                    player_id: 'number',
                    player_name: 'string',
                    player_hitpoints: 'number',
                    player_level: 'number',
                    party_id: '32bit string as Guid'
                },
                {
                    id: '32bit string as Guid',
                    player_id: 'number',
                    player_name: 'string',
                    player_hitpoints: 'number',
                    player_level: 'number',
                    party_id: '32bit string as Guid'
                }
            ] 
        },
    success: true,
    message: 'Succesfull api transaction response.'
}
```

*ERROR*
example
```
    data: null,
    success: false,
    message: 'Some error occurred.'
```