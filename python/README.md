<h3> Schema </h3>

```
[COMPETENCY ID]: 
  [SKILLSET ID]:
    [SKILL ID]:
    - content.md
    - exercise.json
```

<h3> Generate schema </h3>

```
python3 gen_dir.py [file_name].yml 
```
To clear the folder before generating files, use `-d`

```
python3 gen_dir.py [file_name].yml -d
```

<h3> Generate random answers </h3>

```
python3 gen-answer.py [file_name].json 

#Example: python3 gen-answer.py ALG-003/1/1/exercise1.json
```

The GraphQL mutation input will be generated at `log.txt`. 