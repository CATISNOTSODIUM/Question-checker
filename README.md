<h1> Data Serialization (JSON Convention) </h1>
<h2> Front-end </h2>

<h3> Sending response from front-end to back-end </h3>

<h3>Input</h3> 

This JSON format is used when sending data from front-end to back-end to check the answer.


```

{

    "question_id": [competency_id]_[skillset-id]_[skill-id]_[question-index],

    "question_type": [question_type],

    "body": {
        "answer": string, 
        "answer_index": u32,
    }
}


```

The child object `body.answer` and `body.answer_index` are included to add an extra-step verification. (It can be removed later on.) 

<h3>Response</h3> 

To be designed.

<h3> JSON format for displaying question </h3>

This JSON format is used when sending data from back-end to front-end.

<b>General structure</b>

```

{

    "question_id": [competency_id]_[skillset-id]_[skill-id]_[question-index],

    "question_type": [question_type],

    "body": {
        "question":[question],
        "label": [for CAT question],
        "choices": [choices],
    }
}


```


Example: 

```
#index.json
{

"question_id": "ABC-123_456_789_1",

"question_type": "MCQ",

"body": {
    "question": "A flashing red traffic light signifies that a driver should do what?",
    "label": null,
    "choices": [
        "stop", "shutup"
    ]
}

}


```

<h2> Back-end </h2>


The question and answer of each question are stored in the same JSON object as shown in the example below.

```
#JSON template for one problem
{
    "problem_id": "ABC-123_456_789_0",
    "problem_type": "MCQ",
    "body": {
        "question": "A flashing red traffic light signifies that a driver should do what?",
        "label": null,
        "choices": [
            "stop",
            "shutup"
        ],
        "answer_index": 0,
        "answer":"stop"
    }
}


```

The structure is almost similar to JSON format for sending response, but without `answer` and `answer_index`.

<h2> General format</h2>

<h3> 

`question_type` 

 </h3>

Multiple choice: `MCQ` <br>
True false: `TFQ `       <br> 
Categorization: `CAT`  <br>
Error checking: `ERC`   <br>

Even though `MCQ, TFQ, ERC` have the same input-output format, this differentiation is preferred for error handling. Please check out the following templates for clarification.

Noted that `image` feature has not been implemented yet.

<h3> 

`body` 

 </h3>

<h3> 

Multiple choice questions  `MCQ` 

</h3>

```
"body": {
    "question": "A flashing red traffic light signifies that a driver should do what?",
    "label": null,
    "choices": [
        "stop", "shutup"
    ]
}

```

<h3> 

True-false questions  `TFQ` 

</h3>

```
"body": {
    "question": "London bridge is falling down.",
    "label": null,
    "choices": [
        "True", "False"
    ]
}

```

Noted that `"choices": ["True", "False", "Not given]` is still acceptable.

<h3> 

Categorization questions*  `CAT` 

</h3>


```
 "body": {
    "question": "Fruits and veggies classification",
    "label": ["Fruits", "Veggies"],
    "choices": [
        "Apple", "Carrot", "Raddish"
    ]
    }

```

<h3> 

Error-checking `ERC` 

</h3>


```
 "body": {
    "question": "Identify which line should be adjusted to be correct.",
    "label": "int main():\n print("hello world")",
    "choices": [
        "All"
    ]
    }

```

```
 "body": {
    "question": "Identify which line should be adjusted to be correct.",
    "label": "int main():\n print("hello world")",
    "choices": [
        "1", "3", "5"
    ]
    }

```