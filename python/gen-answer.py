import os
import json
import numpy as np
import sys

try:
    file_name = sys.argv[1]
except:
    print("Insert your file path from ./assets/materials.\nExample: file_name  = ALG-003/1/1/exercise1.json")
    exit()
    
if (file_name=="--h" or file_name=="--help" ):
    print("python3 gen-answer.py [file_path]")
    exit()
    
    
#Generate random answer from Exercise.json
#example: file_name  = "ALG-003/1/1/exercise1.json"
user_name = "gen-answer"
root_dir =  os.path.join(os.getcwd(), "../") 
file_path = os.path.join(root_dir, "assets","materials",file_name) 

#load json
questions = None
with open(file_path) as f:
    questions = json.load(f)
    
answer_list = []
target_score = 0
for question in questions:
    answer = np.random.choice(question["body"]["choices"])
    if (answer == question["body"]["answer"][0]):
        target_score+=1
    answer_list.append({
        "questionId": question["question_id"],
        "body":{"answer": [answer]}
    })

formatted_answer_list = ""
for answer in answer_list:
    tmp = '''{
        questionId: "%s",
        body: {
            answer: %s
        }
    }
    '''%(answer["questionId"],str(answer["body"]["answer"]).replace('\'', '"') )
    formatted_answer_list += tmp
    
output = ('''
mutation {
	checkAnswers (
    user: {
      displayName: "%s"
    }
    filePath: "%s",
    payloads: [%s]
  ) {
    correct
    wrong
  	unanswered
    failed
  }
}
''' %(user_name,file_name,formatted_answer_list) )

print("GENERATED INPUT IN log.txt.\nTarget score:",target_score)
f = open("log.txt", "w")
f.write(output)
f.close()