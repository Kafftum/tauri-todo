from flask import Flask, jsonify, request 
from flask_sqlalchemy import SQLAlchemy

app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///database.db'
db = SQLAlchemy(app)

# { "title": todo }

class Todo(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(30), unique=True)

@app.route("/add-todo", methods=['GET', 'POST'])
def add_todo():
    title = request.get_json().get("title")
    db.session.add(Todo(title=title))
    db.session.commit()
    return '', 204

@app.route("/get-todos", methods=['GET'])
def get_todos():
    todos = list(map(lambda q: {'title': q.title}, Todo.query.all()))
    return jsonify(todos)

@app.route("/delete-todo", methods=['GET', 'POST'])
def delete_todo():
    title = request.get_json().get("title")
    todo_to_delete = list(filter(lambda t: t.title==title, Todo.query.all()))
    db.session.delete(todo_to_delete[0])
    db.session.commit()
    return '', 204