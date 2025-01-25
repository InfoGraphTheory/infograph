## info graph
The info graph is a graph-like structure where edges can function as vertices in other edges.
This makes the graph design uniform in structure, descriptive by nature and simplistic when storing. 

## info triple
Info graphs solely consists of one type of primitive which is called info triple.
An info triple can be seen as an special kind of edge. 
It consists of three vertices that is, the two ends and the edge's own vertex, the edge vertex.

## edge vertex
Being able to treat the edge as a vertex has an advantage when it comes to modeling in detail. 
It means that every time an edge is made, a new possible vertex is added to the graph. 
Hence for each new edge, *n* new edges can potentially be drawn, where *n* it the number of vertices in the graph.

## info table
Since an info graph consists of info triples and info triples consists of three vertices,
an info graph can be written in a simple table format with three columns which we call an info table. 
The first column in the info table is for primary key which is the identifier of the edge vertex.
The other columns holds the identifiers of the adjacent vertices of the info triple.
