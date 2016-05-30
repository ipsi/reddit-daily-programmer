Random thoughts on this.

```
A B C D E F
A C 5
A B 10
D A 5
D E 10
C E 10
E F 15
B F 20
```

```
Node {
  name
  Link[]
}

Link {
  res
  Node
}

Node
- name: A
  links:
  - resistance: 10
    Node:
    - name: B
      links:
      - resistance: 50
        Node:
        - name: C
          links: []
  - resistance: 30
    Node:
    - name: B
      links:
      - resistance: 50
        Node:
        - name: C
          links: []

APPEND A.LINKS
POP A.LINKS (size > 1 - funky math)
ITERATE A.LINKS
0
ADD 1/X to total
APPEND A.LINKS[0].LINKS
1
ADD 1/X to total
APPEND A.LINKS[1].LINKS (fail - already present - need equality operator)
POP B.LINKS (size = 1 - add to result)




APPEND A.LINKS
POP A.LINKS
A.LINKS > 1
ITERATE
0
APPEND A.LINKS[0].LINKS
```













```
     +--(05)--[C]--(10)--+
     |                   |
     |                   +--[E]--(15)--+
     |                   |             |
[A]--+--(05)--[D]--(10)--+             +--[F]
     |                                 |
     |                                 |
     |                                 |
     +--(10)--[B]---------(20)---------+

Simplify:

     +--(15)--+
     |        |
     |        +--[E]--(15)--+
     |        |             |
[A]--+--(15)--+             +--[F]
     |                      |
     |                      |
     |                      |
     +--(10)--[B]---(20)----+

Simplify:

     +--(7.50)--[E]--(15.0)--+
     |                       |
[A]--+                       +--[F]
     |                       |
     +--(10.0)--[B]--(20.0)--+

Simplify:

     +--(22.5)--+
     |          |
[A]--+          +--[F]
     |          |
     +--(30.0)--+

Simplify:

[A]--(12.86)--[F]

12.857142857

A.LINKS
0
```

















```
                 +-(15)--+
     +--(05)--[C]+-(10)--+
     |           +-(25)--+
     |                   +--[E]--(15)--+
     |                   |             |
[A]--+--(05)--[D]--(10)--+             +--[F]
     |                                 |
     |                                 |
     |                                 |
     +--(10)--[B]---------(20)---------+

while (true) {
	process_node(a)
	if a.links.size == 1 && a.links[0].links.size == 0
		break
}

println a.links[0].res

// SIMPLIFYING ASSUMPTION - always start from A. A has 0 backlinks, and 1 or more outgoing links.
// SIMPLIFYING ASSUMPTION - only one terminal node
// SIMPLIFYING ASSUMPTION - layout is alphabetial (A -> B -> C)

process_node(node) {
	// [A]--(1)
	if node.links.size == 1 {
		// [A]--(1)--[B]
		if node.links[0].backlinks.size == 1 && node.links[0].links.size == 0 {
			// DONE!
		}
		// [A]--(1)--[B]--(1)--[C]...
		else if node.links[0].backlinks.size == 1 && node.links[0].links.size == 1 {
			// [A]--(2)--[C]...
			node.links[0].backlinks[0] = node.backlinks[0]
			node.backlinks[0] = node.links[0].backlinks[0]
			node.backlinks[0].res += node.res
			process_node(node.links[0])
		}
		else {
			// If we get here, then one of our callers will deal with siblings.
		}
	}
	//      +--(05)--[C]
	//      |
	// [A]--+--(05)--[D]
	//      |
	//      +--(10)--[B]
	else {
		maxLinkCount = node.links.reduce(0, { i, n -> max(i, n.links.size) })
		maxBacklinkCount = node.links.reduce(0, { i, n -> max(i, n.backlinks.size) })
		//      +--(05)--[C]--+
		//      |             |
		// [A]--+--(05)--[D]--+--[E]
		//      |             |
		//      +--(10)--[B]--+
		if maxLinkCount == 1 && maxBackLinkCount == 1 && uniques(node) == 1 {
			// squeeze nodes
			res = 1 / node.links.reduce(0, { i, n -> i += (1 / n.res)})
			name = node.links.map({ it.name }).join("/")
			nnode = new Node(name, res)
			node.links[0].backlinks.clear
			node.links[0].backlinks.add(nnode)
			node.backlinks[0].links.clear
			node.backlinks[0].links.add(nnode)
		}
		map nbygc // nodes by children
		for n in node.links {
			process_node(n)
			nbygc[n.links[0]] << n
		}

		for (key, value) in nbygc {
			if value.size > 1 && value.backlinks[0].links.size > 1 {
				squeeze_nodes(value)
			}
		}
	}
}

uniques(node) {
	set nodes
	for n in node.links { nodes.add(n) }
	return nodes.size
}
```
