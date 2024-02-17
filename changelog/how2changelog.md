# How to make a character data doc
1. Make sure you have a downloaded copy of the vanilla scripts from wuboy's repository (smashline)
2. run py statistics.py {char_name} and follow all prompts
3. open output.md, edit it to where its accurate and makes sense (some specials need more editing than others, all projectiles need their info put into the move, projectile stats are on the bottom)
4. save as {character}.md within the changelog

Tip: Check the file [here](https://markdownlivepreview.com) in order to make sure it looks right


MAKE SURE TO DOUBLE CHECK THE PROJECTILE VALUES LISTED ARE CORRECT IF THEYRE CHANGED FROM VANILLA


If theres no FaF written under it (not even FaF: --) then something has gone wrong, find out what its FaF is (either ingame or with export_yamlist + a little bit of maths) and add it.

# How to update a character data doc (for your PRs)
1. do same as above except instead of replacing the entirety of the character's .md file, only the moves changed within your PR
