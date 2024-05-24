<h1> Graph QL input example </h2>

<h2> Query </h2>

```
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

```
query {
  fetchContent(
    filePath: "ALG-003/1/2/content.md"
  )
}
```


```
{
  getQuestions(filePath: "ALG-003/1/2/exercise1.json") {
    questionId
    body {
      question
    }
  }
}
```


<h2> Mutation </h2>

```
mutation {
  checkAnswer (
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
