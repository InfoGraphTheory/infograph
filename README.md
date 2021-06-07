# info graph
The info graph is a graph-like structure where edges can function as verticies in other edges.
This makes the graph design uniform in structure, descriptive by nature and simplistic when storing. 

# info triple
Info graphs solely consists of one type of primitive which is called info triple.
An info triple can be seen as an special kind of edge. 
It consists of three verticies that is, the two ends and the edge's own vertex, the edge vertex.

# edge vertex
Being able to treat the edge as a vertex has an advantage when it comes to modeling in detail. 
It means that every time an edge is made, a new possible vertex is added to the graph. 
Hence for each new edge *n* new edges can potentially be drawn, where *n* it the number of verticies in the graph.

# info table
Since an info graph consists of info triples and info triples consists of three verticies,
an info graph can be written in a simple table format with three rows which here is called an info table. 
The first row in the info table is for the edge vertex which is the identifier.
