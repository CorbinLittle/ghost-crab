Goals 
1. improve at the rust programming language. my first major project tought me how to do things in rust. I hope for this project to teach me how to do things the right way.
2. have the best chess engine in the rust programming language. There is not a lot of active development of chess engines wirtten in rust. I think this gives me a reasonable chance of becoming the best chess engine written in rust.
3. after the engine itself is reasonably powerful I want to work on making this easier to integrate with other programs, this will mean implementing a plugin system to create personalities and change adjust difficulty as well as improving on evalution data.
   this will make it better for adding to existing chess apps written in rust.

todo list for week one
1.	implement move piece for board

completed weekly taks
1. switch from weird enum bassed representation for tiles to a more ergonomic Vec<Vec<(Piece, Color)>> bassed representation(not yet commited because it breaks other parts of the program which will be fixed by implementing other items in the todo list)
2. add a readme
3. add a license
4. switch form vector bassed board representation to array bassed board reperesentation
5.	Implement From<i8> for Square for your conversion from the numbers


unfortunatly I will not be accepting merge requests at the moment. My main goal right now is to improve personaly with this project. once I feel I have reached my goal of generally being able to do things the right way in this language I will open in up 
to the community to contribute. I would however greatly appreciate any recomendations on how I could improve this project so feel free to open up an issue. other then that I am posting weekly on r/rust to get feedback on how to improve this project. 
