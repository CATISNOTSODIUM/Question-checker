<h2> GraphQL Question-Checker Usage Description </h2>
<h3> General description </h3>

This program acts as a bridge between a GraphQL front end and question database by fetching questions and submitting answers. 

In this current state, the questions are temporarily stored inside `asset/material` directory as `.json` files, and some codes are written in such manner to handle this implementation. In case you want to fetch questions from database, you can replace `file_path` with context (`&Context<'ctx>`). However, these features have not yet been separately implemented.


<h3> Code structure </h3>

```
graphql
├── manual.md (This file)
├── mod.rs
├── schema.rs (Bridging between axum and GraphQL operations)
│
│   [Data types]
├── question_types.rs 
├── response_types.rs
├── types.rs (General type: e.g. Users)
│
│   [GraphQL Operations]
├── query
│   ├── fetch_content.rs
│   ├── fetch_question.rs
│   └── mod.rs
└── mutation 
    ├── check_answer.rs
    └── mod.rs

```
The details on the implementation are written separately in the file.

<h2>GraphQL Operations</h2>

Here is the overall structure of GraphQL operations.

```
QUERY
 ├── fetch_question.rs
 │    - get_question (fetch specific question from question id)
 │    - get_questions (fetch all questions from the desired endpoint)
 └── fetch_content.rs
      - fetch_content (fetch markdown file as raw text)
 
 MUTATION
 └── check_answer.rs
     - check_answer (check answer for specific question id)
     - check_answers (check answers of all questions and submit the result to database)
```


<h3>Instruction for querying questions</h3>

<h4> Querying single question </h4>

```
#graphql
#Fetch question MCQ_1_0.

query {
  getQuestion(
    filePath:"ALG-003/1/2/exercise1.json",
    questionId:"MCQ_1_0"
  )
  {
    questionId
    body {
      question
      choices
    }
  }
}
```
<h4> Querying all questions </h4> 

```
#graphql
#Fetch question MCQ_1_0.

query {
  getQuestions(filePath: "ALG-003/1/2/exercise1.json") {
    questionId
    body {
      question
    }
  }
}
```

<h3>Instruction for submitting answers</h3>
<h4> Check single question </h4>


```
mutation {
  checkAnswer (
    user: {
      displayName: "Jame"
    }
    filePath: "ALG-003/1/2/exercise1.json",
    payload: {
      questionId: "TFQ_1_1",
      body: {
        answer: "True"
      }
    }
  )
}
```

<h4> Check all question </h4>


```
mutation {
 checkAnswers (
    user: {
      displayName: "gen-answer"
    }
    filePath: "ALG-003/1/1/exercise1.json",
    payloads: [{
        questionId: "MCQ_1_0",
        body: {
            answer: ["shutup"]
        }
    }
    {
        questionId: "TFQ_1_1",
        body: {
            answer: ["True"]
        }
    }
    {
        questionId: "MCQ_1_2",
        body: {
            answer: ["Phraya Manopakornnititada"]
        }
    }
    {
        questionId: "MCQ_1_3",
        body: {
            answer: ["Paparazzi"]
        }
    }
    {
        questionId: "MCQ_1_4",
        body: {
            answer: ["Inaudible Sounds"]
        }
    }
    {
        questionId: "MCQ_1_5",
        body: {
            answer: ["A complex number"]
        }
    }
    {
        questionId: "MCQ_1_6",
        body: {
            answer: ["Damrong Thipyotha"]
        }
    }
    {
        questionId: "MCQ_1_7",
        body: {
            answer: ["Himself"]
        }
    }
    {
        questionId: "MCQ_1_8",
        body: {
            answer: ["To find an entry in an ordered list"]
        }
    }
    {
        questionId: "TFQ_1_9",
        body: {
            answer: ["False"]
        }
    }
    ]
  ) {
    correct
    wrong
   unanswered
    failed
  }
}
```