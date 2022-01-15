# queryer

So, let's design a library that can use SQL queries on any data source and get the results? Of course, as an MVP (mimimu viable product), we only support SQL queries on CSV for the time being. Not only that, we also hope that this library can be given to Python3 and Node.js.

We try to do a transform from CSV to DataFrame. We use `polars` lib to do that.

Finally the core problem to be solved in "SQL query on CSV and other sources" becomes how to convert one ast (SQL AST) into another ast (Dataframe AST).

![1](https://static001.geekbang.org/resource/image/dd/72/ddd4995deecc4b7897bf73beb0e2cb72.jpg?wh=1920x1266)

Let's get it done.

---