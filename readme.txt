Thank you for downloading Ultimate S!


-Instructions-
1. Run "Ultimate S Setup Tool.exe" (included with your download)
2. Choose between Full (all characters have their Ultimate S Movesets) and lite (You choose which characters have their movesets, EXPERIMENTAL)
3. Follow the instructions from the program
4. The program should give you options on how to put it onto your nintendo switch/emulator:
a. Send to a specific folder (used for emulators and people with SD card readers). Automatically sets up your SD card/folder too!
b. Send via FTP (used for switch). Requires FTP set up on your switch. Automatically sets up your SD card too!
c. Manual. Does not set your SD card up automatically, copy your atmosphere and ultimate folders to the root of the SD card.
(Note: You don't need to specify which added characters to include, they're automatically there already!)

WARNING: Any character problems caused by disabling characters, 
and any compatibility with other smash ultimate character mods is up to YOU (the end user) and the respective character mod creators to fix.
Ultimate S is built to work as a whole package, and the setup tool is simply a workaround.


This mod assumes you have the latest version of arcropolis and therefore does not include it in the download. If you haven't set up smash ultimate modding as a whole, here's some wonderful guides!
Setting up CFW on switch - https://switch.homebrew.guide/
Modding Smash Ultimate on switch - https://gamebanana.com/tuts/12827
Modding Smash Ultimate on yuzu - https://youtu.be/uKKQl7QFBMA


-Dependecies-
Dependencies should come with the download, and the setup tool should update them for you also, 
but if for some reason they aren't there, or your game is crashing, try updating these.

libparam_config.nro - https://github.com/CSharpM7/lib_paramconfig
libcsk_collection.nro - https://gamebanana.com/mods/499008 (just grab the nro file from this. Do NOT grab the files in ultimate/mods from this download)
libstage_config.nro - https://github.com/ThatNintendoNerd/stage_config
libarcropolis.nro - https://github.com/Raytwo/ARCropolis
libnro_hook.nro - https://github.com/ultimate-research/nro-hook-plugin
libsmashline_plugin.nro - https://github.com/HDR-Development/smashline/releases/latest
skyline - https://github.com/skyline-dev/skyline
arena latency slider - https://gamebanana.com/mods/496679

Place the plugins in "/atmosphere/contents/01006A800016E000/romfs/skyline/plugins"
Place skyline in "/atmosphere/contents/01006A800016E000"


-Common Issues-
Q: "My game is crashing! What do I do?"
A: There could be many things that could be causing this! Try going down this list to make sure that it isn't a user error:
1. Make sure your version of smash ultimate is update 13.0.2 (other updates do not work)
2. Make sure training modpack is not enabled (this means no training modpack plugin, and no param hook plugin)
3. Make sure your plugin folder is CLEAN (this means the only plugins in the folder are libparam_config, libcsk_collection, libstage_config, libarcropolis, libnro_hook, libsmashline_plugin and libarena_latency_slider).

Q: "The character's attacks are messed up! They're doing their vanilla animations!"
A: This sounds like you have a mod conflict. If you're on switch, this is easy because arcropolis will tell you this, and create a conflicts.json for you (which you can use to disable the conflicting mods).
If you're on Yuzu, this is a more involved process. If you really just want Ultimate S to work, empty out your ultimate/mods folder and ensure its just "Ultimate S Arcropolis" and "Ultimate S Stages" there.
Check that this boots up and works just fine. If it does, it's definitely a mod conflict. Slowly add in your skin mods until you find where the conflict is, its usually a conflict where the skin mod edits a character's 
motion folder (example: https://gamebanana.com/wips/75639) , or a projectile's model (example: https://gamebanana.com/mods/350254). You'll have to delete the folders which conflict with Ultimate S or disable those mods.

Q: "The game is going too fast!"
A: You're likely on Yuzu. Make sure game speed is set to 50%, by going to emulation -> config -> system -> limit speed percent (on yuzu) and setting it to 50%, and this should make your game run at regular speeds.
If you're on Ryujinx, try download RTSS Rivatuner here: https://www.guru3d.com/download/rtss-rivatuner-statistics-server-download
and follow these instructions: https://www.guru3d.com/page/how-to-use-rivatuner-to-limit-fps

Q: "The game is just vanilla!"
A: Try going through these steps:
1. Check your title screen, does it say Arcropolis in the top right? If not, arcropolis isn't running, make sure you have skyline AND arcropolis installed correctly.
2. Check your title screen, what version is it? Make sure it is 13.0.2, and no other version.
3. Make sure that Ultimate S is enabled (either by mod manager on the switch, or inside of the mods folder on Yuzu).
4. Make sure you ran "Ultimate S Setup Tool.exe" and chose your preferred options (you can check in ultimate/ult-s to see which characters you have enabled)
5. If you are on emulator, make sure that there is a file called "legacy_discovery" at ultimate\arcropolis\config\2470593114292646594\15904440331188662786
If there isn't, you can make one by creating a file (no extension on the end) with "True" written in it


Q: "I want to remove X character's changes"/"I want to just have Y character"
A: Run "Ultimate S Setup Tool.exe", it allows you to pick and choose which ultimate S movesets you have.



Any other issues or queries should be directed to the discord server (https://discord.gg/YXaPzC5WGd)

Have fun!



Credits:
The people that make this mod what it is!

-Core Dev Team-
chrispo - Coding, Animation, Weapon Modelling, Character Balance, Stages, UI
Challat - Animations, Weapon and Character Modelling, Rigging
Eriizer - UI, renders
MonkeyMike - Character Balance
mRama - Coding, Animation, Weapon Modelling, UI, original Sandbag mod, original Masked Man Mod

-Major Contributors-
Angst - Stages 
C# - Stages 
Ella - Character Balance

-Special Thanks to-
AParticularUser - Ridley Dair Animation
blujay - Stage Select Screen, Additional Control Buttons, 1 Frame Delay Removal, Arena Latency Slider
Chaos.A - Mario Nair animation
CoolSonicKirby - Porting of Additional Control Buttons
Devory - Colloseum, Gamecube Stages, Ready to Fight UI
Ginn. - Original Paper Mario Stage
iOZ_MintyFresh - Hexagon UI
Joben - Stage Select Screen
Kybbler - Bomberman Victory Screen Camera
LN_310 - ZSS Model, Rauru's Blessing
LKE Studios - DK Utaunt animation
LuXejin - Sonic Dtaunt animation
MaroonRed - Mario nair sound
Mayamia - Pause Screen UI, Loading Screen Animation
mokl - Wolf utilt animation
Nano - Masked Man Model + Render
Naps - Sonic nair animation
Nozzly - CPU adjustments
PhazoGanon - Animations
Saminate - Dr. Mario animations
SneakyKNG - Toad Rigging
SquidEnthusiast - Toad Render
Star Warrior - Stages 
SushiiZ - Richter Dash Attack
willy_crescent - Ganon's Tower, General Stage Help
WuBoytH - General Major help
