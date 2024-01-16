# Preface
Catalog ALL of your changes, pull requests should CHANGE the changelog within the PR.
I recommend you look at mario's (currently unfinished lmao) changelog file for reference 
Theres a copy of the default fighter_param for reference at scripts/data/default.xml

# Guidelines
1. Make sure you label changes as [vanilla value] > [new value]
2. Label changes as buffs/nerfs/sidegrade/reworks/aesthetic/new move to make it easier for people to read and understand. For example:
    - [+] FaF reduced 90 > 25 (this is a buff)
    - [-] BKB increased 20 > 30 (this is a nerf, discretion may be needed for kb changes to determine if its a nerf or not)
    - [/] Angle increased 50 > 65 (this is a sidegrade, for when it opens some options and closes some others)
    - [&] Now jump cancellable (this is a rework, intended for adding new stuff to a move)
    - [!] Now a new move - Galaxy spin (this is a new move!)
    - [*] Utaunt has a new animation (this is aesthetic)
3. When talking about a new move, do it like so:
## Nair
[!] Now a new move - Galaxy spin
Mario twirls in the air using his galaxy spin
| Stat | Value |
| ------------- |:-------------:|
| Startup  | 8 |
| Active frames  | 8-18 |
| Damage | 9.0% |
| Angle | 361 |
| BKB | 30 |
| KBG | 95 |
| FaF | 38 |
| Landing Lag | 6 |
| Autocancel Frames | 1-2/33+ |
| Notes | Stalls once per Airtime |

or like so:
## Dtilt
[!] Now a new move - Scrimblo Boom
Mario explodes
| Stat | Value |
| ------------- |:-------------:|
| Startup  | 32/20 (Regular/Powered Up) |
| Active frames  | 32.. |
| Damage | 3.0%/8.0%/12.0% (close/mid/far) |
| Angle | 361/45/10 (close/mid/far) |
| BKB | 100 (Set) |
| KBG | 100 |
| FaF | -- |
| Landing Lag | -- |
| Autocancel Frames | -- |
| Notes | Lasts until mario lets go of the attack button |
| Notes | Jump cancellable on frame 4 |
| Notes | Airdodge cancellable on frame 80 |
| Notes | Transitions into Jab 2 on frame 6 |
| Notes | If mario crouches for 5 seconds beforehand, powered up version  |

4. When listing hitboxes with different values either: 
    - start from lowest ID (highest_priority/mid_priority/low_priority)
    - start from the earliest frame (early/late)
    - start from the earliest hit (hit 1/hits 2-7/hit 8)
If there is a combo of these split them up:
    - [-] Nair 1 damage reduced 10.0%/7.0% (early/late) > 9.0%/5.0%
    - [-] Nair 2 damage reduced 35.4%/7.0% (early/late) > 3.0%/5.0%

5. when making a changelog, make it in this order (pretty please):
# Attributes
log all the changed stuff here (ignore footstool stuff as that'll go in the engine changes)
# Grounded Normals
## Jab
### Jab 1
### Jab 2
### Jab 3 (etc. etc.)
## Ftilt
### if theres an Ftilt 2 split them up like jab
## Dtilt
## Utilt
## Dash Attack
## Fsmash
## Usmash
## Dsmash
# Aerials
## Nair
## Fair
## Bair
## Uair
## Dair
# Specials
## Neutral Special
### If it needs to be split into parts thats cool
## Side Special
## Up Special
## Down Special
# Grabs
## Standing Grab
## Dash Grab
## Pivot Grab
## Pummel
## Fthrow
## Bthrow
## Uthrow
## Dthrow
# Misc.
you can just put stuff in text here no headings for misc (stuff like taunts, new idles, etc. etc.)
