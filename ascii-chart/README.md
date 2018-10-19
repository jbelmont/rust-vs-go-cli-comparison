In order to compile the C Program run:

`gcc ascii_table.c -o ascii_table`

Then run the following to binary executable to get the ascii characters but no control or whitespace characters.

`./ascii_table | sed "s;.;\'&\'\,;g" | pbcopy`

The sed command wraps each line in the `'` in single quotes and adds a comma `,` at the end and the pbcopy is a mac command to copy all of the standard output into the system clipboard.