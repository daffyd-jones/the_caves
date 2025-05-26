# todo

- Trade Npcs - fix bugs
- Puzzles
  - Teleport
  - Maze X
  - Inverted
- Compass ~
- Remove items when wall
- ShopNPCs X
- cursor limit
- enchantments
- notebook
  - add with SpawnNPC
  - update with Puzzle
  - add with Settlement

- make item sub-enum
- add plant items
- add herbalist NPCs
  - add convos intro, each plant explanation
  - add functionality for identifying plants
- add cauldron
  - add EnvInter
  - add icon/gui
  - implement interaction
    - choose conversion
      - plyr pick plants to use || game provides possible options
    - conversion of xPlants = 1Potion
    - rem plants, place potion
  - cauldrons in all/some settles

## TaskNPC

- convo for task
- content for task
  - pos
  - type: item, trigger
  - store content
- trigger interaction after convo
- interaction
  - check for completion
  - provide response
  - optional give item
  - implement give item
    - show inventory, pick, check, take || check, pick, take

- [ ] convert ShopNPCs to EnvInter?
- [ ] test area animate
- [+] refactor project
- [+] put icons in menu for weapons
- [+] fix trading item handling
- [+] make settlements hidden until found
- [+] SpawnNPCs now attached to node puzzle content, directing to existing puzzles
- [+] make experience handling/upgrading

- [ ] implement plants
  - [+] implement constructors
  - [ ] implement placement
  - [+] implement cauldron
  - [ ] implement herbalist
    - [ ] create NPC
    - [ ] implement convo
    - [ ] implement plant identification

- [ ] implement TaskNPCs - for retrieving items (will require persistance, best for settles/features)
  - [ ] create NPCs
  - [ ] implement convos
  - [ ] create Task
  - [ ] implement task creation
  - [ ] implement task completion
  - [ ] implement completion convo/reward

- [ ] add notes for settlements, cave knowledge, discoveries (puzzles, intresting features)
- [+] add fight mode choice to fight start
- [+] update encounter gui
- [+] implement ascii
- [ ] implement features - creation/nodemap placement
  - [ ] implement Hermit
- [ ] menus - main/ingame/death
- [ ] save/load
- [ ] enemy/weapon grading
- [ ] maybe renown good/evil?

```
Ŧ ŧ ƍ ǫ Ǫ ǭ Ǭ ȩ ȣ Ȣ ȩ ḉ ḝ ọ Ọ ˚ ϙ Ϙ  ৭ ૦౸ ⚷	⚸ 
	†
 ǫ ǭ ọ ϙ           
               
⁘	⁙	⁚	⁛	⁜	⁝	⁞
※

⛶ ☩ ⊹ 
ଽஃ  ౽	౾ ෴෴෴෴෴෴෴     ෴
ஈஈஈஈஈஈஈஈஈஈஈ   
ண
܀	܁	܂	܃	܄	܅	܆	܇	܈	܉	܊	܋	܌	܍

ᘈ	ᘉ	ᘊ	ᘋ	ᘌ	ᘍ

ᘉ ᘈ ᘍ ᘊ          

 ᘉᘍᘊᘈ                                                                               
 ᘈᘊᘍᘉ                                                                               
 ᘉᘍᘊᘈᘉᘍᘊᘈᘉᘍᘊᘈ                                                                    
 ᘈᘊᘍᘉᘈᘊᘍᘉᘈᘊᘍᘉ                                                                    
 ᘉᘍᘊᘈᘉᘍᘊᘈᘉᘍᘊᘈ      ᘉᘍᘊᘈᘈᘍᘊᘉᘉᘍᘊᘈ                                                                         
 ᘈᘊᘍᘉᘈᘊᘍᘉᘈᘊᘍᘉ      ᘈᘊᘍᘉᘉᘊᘍᘈᘈᘊᘍᘉ                                                                         
                   ᘈᘍᘊᘉᘉᘍᘊᘈᘈᘍᘊᘉ                                                         
                   ᘉᘊᘍᘈᘈᘊᘍᘉᘉᘊᘍᘈ                                                         
                                                                                
 ᘈᘍᘊᘉ                                                                               
 ᘉᘊᘍᘈ                                                                               
 ᘈᘍᘊᘉᘈᘍᘊᘉᘈᘍᘊᘉ                                                                    
 ᘉᘊᘍᘈᘉᘊᘍᘈᘉᘊᘍᘈ                                                                    
 ᘈᘍᘊᘉᘈᘍᘊᘉᘈᘍᘊᘉ                                                                               
 ᘉᘊᘍᘈᘉᘊᘍᘈᘉᘊᘍᘈ                                                                               
                    
                    
ᘉ ᘈ ᘍ ᘊ          
                    
 ᘊᘈᘊᘈ   ᘍᘉᘍᘉ   ᘉᘊᘉᘊ   ᘈᘍᘈᘍ   ᘍᘈᘍᘈ        
 ᘉᘍᘉᘍ   ᘈᘊᘈᘊ   ᘍᘈᘍᘈ   ᘊᘉᘊᘉ   ᘉᘊᘉᘊ     
 ᘊᘈᘊᘈ   ᘍᘉᘍᘉ   ᘉᘊᘉᘊ   ᘈᘍᘈᘍ   ᘍᘈᘍᘈ       
 ᘉᘍᘉᘍ   ᘈᘊᘈᘊ   ᘍᘈᘍᘈ   ᘊᘉᘊᘉ   ᘉᘊᘉᘊ       
                    
 ᘉᘍᘊᘈᘈᘍᘊᘉᘈᘍᘈᘍᘉᘍᘊᘈᘈᘍᘊᘉᘈᘍᘈᘍ                   
 ᘈᘊᘍᘉᘉᘊᘍᘈᘊᘉᘊᘉᘈᘊᘍᘉᘉᘊᘍᘈᘊᘉᘊᘉ                   
 ᘍᘉᘍᘉᘊᘈᘊᘈᘉᘊᘉᘊᘍᘉᘍᘉᘊᘈᘊᘈᘉᘊᘉᘊ                   
 ᘈᘊᘈᘊᘉᘍᘉᘍᘍᘈᘍᘈᘈᘊᘈᘊᘉᘍᘉᘍᘍᘈᘍᘈ                   
 ᘉᘍᘊᘈᘈᘍᘊᘉᘈᘍᘈᘍᘉᘍᘊᘈᘈᘍᘊᘉᘈᘍᘈᘍ                   
 ᘈᘊᘍᘉᘉᘊᘍᘈᘊᘉᘊᘉᘈᘊᘍᘉᘉᘊᘍᘈᘊᘉᘊᘉ                   
 ᘍᘉᘍᘉᘊᘈᘊᘈᘉᘊᘉᘊᘍᘉᘍᘉᘊᘈᘊᘈᘉᘊᘉᘊ                   
 ᘈᘊᘈᘊᘉᘍᘉᘍᘍᘈᘍᘈᘈᘊᘈᘊᘉᘍᘉᘍᘍᘈᘍᘈ                   
                    
                    
                    
                    
                    
                             
   *            '████    ████         :  ████      *         ·   ,      ⁂▓▓▓▓      '  ·  ▓▓▓▓            
▓▓▓▓▓🬱      '    ████    ████████ᘊ       ᘈᘈᘈᘍ███████🬱        :         ⁂⁂▓▓▓▓  ·      :· ▓▓▓▓    🬶▓▓▓▓▓▓▓
▓▓▓▓▓▓           ████ :  ████████ᘊ    *     ᘍ████████⁂  ,      ,    '  ⁂⁂▓▓▓▓            ▓▓▓▓  : ▓▓▓▓▓▓▓▓
▓▓▓▓▓▓·*     , · ████  ' ████████ᘊ : '      ౸████████⁂                  ⁂▓▓▓▓  *       · ▓▓▓▓    ▓▓▓▓▓▓▓▓
▓▓▓▓▓▓        *  🭕███    🬊███████ᘊ  , :     ᘍ████████⁂ ''  :    ⁂⁂⁂⁂⁂   ⁂▓▓▓▓           :🬁▓▓▓   ,▓▓▓▓▓▓▓▓
        ,          ⚶ *    ᘈᘈ౸ᘈᘈᘈᘈᘈ          ᘈᘈᘈᘈᘈᘈ౸ᘈᘈ    ████   ⁂▓▓▓▓⁂ ⁂⁂▓▓▓▓ '   '     :  '   ⁂⁂⁂⁂⁂⁂⁂⁂⁂⁂  
 '       ·           '               *   *               ████   ⁂▓▓▓▓⁂ ⁂⁂▓▓▓▓        ,       ỏ     ,     
         *  , ,        *                     :*       : *████   ⁂▓▓▓▓⁂ ⁂⁂▓▓▓▓ :   ,                  ,   
 ·:: '     ⚶    ⚶         ᘉᘉᘉᘉ౸ᘉᘉ,    ·     ᘍᘉᘉ౸ᘉᘉ౸ᘉ     ████⁂⁂⁂⁂▓▓▓▓⁂⁂⁂⁂▓▓▓▓,  ,:  *      ·           ' 
 ·  :    🬵▓▓▓███████🬳    🭄███████ᘊ          ᘍ███████🬏    ████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ,  🬵▓▓▓▓▓▓▓  :'   ·    ▓▓▓▓
 *      *▓▓▓▓████████    ████████ᘊ        * ᘍ████████    ████▓▓▓▓▓▓▓🬗▓▓▓▓▓▓▓▓   ,▓▓▓▓▓▓▓▓        · ' ▓▓▓▓
        ⚶▓▓▓▓████████    ████████ᘊ'         ᘍ████████   *████▓▓▓▓▓ඉ▓▓▓▓▓▓▓▓▓▓  : ▓▓▓▓▓▓▓▓,   '       ▓▓▓▓
  '   ·  ▓▓▓▓████████    ████████ᘊᘉ౸ᘉ ·  ᘉᘉᘉᘍ████████  ··🬊███▓▓▓▓▓▓ඉ▓▓▓▓▓▓▓▓🬆  * ▓▓▓▓▓▓▓▓   ·   ·    ▓▓▓▓
    ·    ▓▓▓▓     **             ████    ████     ,              ▓▓▓▓          ,            · '          

〄

ϙ ৭ ƍ ǫ ⁂  ℧ ⊤

⏣ ⎊ ⎉ ⍟ ⌾ ⌬ ◉ ◍ ◎ ✇             

⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶
⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶⚶  ⚶

⁂⁂⁂⁂⁂⁂⁂⁂⁂⁂
⁂⁂⁂⁂⁂⁂⁂⁂⁂⁂


≊	≋  ~~~

  ~~~~~~               
 ~~~~~~~~~                
  ~~~~~~~~~               
  ~~~~~~~               
   ~~~~~              
                 
   ≋≋≋≋≋             
  ≋≋≋≋≋≋≋≋               
 ≋≋≋≋≋≋≋≋≋≋≋                
 ≋≋≋≋≋≋≋≋≋≋≋≋≋                
  ≋≋≋≋≋≋≋≋≋≋               
   ≋≋≋≋≋≋≋≋              
     ≋≋≋≋            
                 
                 

∭∭∬∭∭∭
∭∭∭∬∭∭
⋙⋙⋙⋙⋘⋘⋘⋘

⋘⋙⋘⋙⋘⋙⋘⋙           

─	━	│	┃	┄	┅	┆	┇	┈	┉	┊	┋	┌	┍	┎	┏
┐	┑	┒	┓	└	┕	┖	┗	┘	┙	┚	┛	├	┝	┞	┟
┠	┡	┢	┣	┤	┥	┦	┧	┨	┩	┪	┫	┬	┭	┮	┯
┰	┱	┲	┳	┴	┵	┶	┷	┸	┹	┺	┻	┼	┽	┾	┿
╀	╁	╂	╃	╄	╅	╆	╇	╈	╉	╊	╋	╌	╍	╎	╏
═	║	╒	╓	╔	╕	╖	╗	╘	╙	╚	╛	╜	╝	╞	╟
╠	╡	╢	╣	╤	╥	╦	╧	╨	╩	╪	╫	╬	╭	╮	╯
╰	╱	╲	╳	╴	╵	╶	╷	╸	╹	╺	╻	╼	╽	╾	╿

╭─┄┄─╮
├ ┐  │
│ │  ┊
│ │  │
╰─┴──╯        
           
▀	▁	▂	▃	▄	▅	▆	▇	█	▉ ▊ ▋	▌	▍ ▎	▏▐ ░ ▒ ▓	▔	▕	▖	▗	▘	▙	▚	▛	▜	▝	▞	▟

▢	▣	▤	▥	▦	▧	▨	▩

🮘	🮙

🬤	🬥 🬪 🬗 ▚ ▞
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒🬤▒▒▒▒▒▒🬪▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒🬥▒▒
▒▒▒▒▒▒🬤▒▒▒▒▒▒▒▒▒▒
▒▒🬥▒▒▒▒▒▒▒🬗▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒🬤▒▒▒▒▒▒▒▒🬤▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒

🜁	🜂	🜃	🜄	🜅	🜆	🜇	🜈	🜉	🜊	🜋	🜌	🜍	🜎	🜏
🜐	🜑	🜒	🜓	🜔	🜕	🜖	🜗	🜘	🜙	🜚	🜛	🜜	🜝	🜞	🜟
🜠	🜡	🜢	🜣	🜤	🜥	🜦	🜧	🜨	🜩	🜪	🜫	🜬	🜭	🜮	🜯
🜰	🜱	🜲	🜳	🜴	🜵	🜶	🜷	🜸	🜹	🜺	🜻	🜼	🜽	🜾	🜿
🝀	🝁	🝂	🝃	🝄	🝅	🝆	🝇	🝈	🝉	🝊	🝋	🝌	🝍	🝎	🝏
🝐	🝑	🝒	🝓	🝔	🝕	🝖	🝗	🝘	🝙	🝚	🝛	🝜	🝝	🝞	🝟
🝠	🝡	🝢	🝣	🝤	🝥	🝦	🝧	🝨	🝩	🝪	🝫	🝬	🝭	🝮	🝯
🝰	🝱	🝲	🝳

🀰🀰🀰🀰🀰🀰 
▒
🁢🁢🁢🁢🁢🁢
🁢🁢 

﹇﹈﹉﹊﹋﹌﹍﹎﹏
︹︺︻︼

︻
︻

🬀	🬁	🬂	🬃	🬄	🬅	🬆	🬇	🬈	🬉	🬊	🬋	🬌	🬍	🬎	🬏

🬐	🬑	🬒	🬓	🬔	🬕	🬖	🬗	🬘	🬙	🬚	🬛	🬜	🬝	🬞	🬟

🬠	🬡	🬢	🬣	🬤	🬥	🬦	🬧	🬨	🬩	🬪	🬫	🬬	🬭	🬮	🬯

🬰	🬱	🬲	🬳	🬴	🬵	🬶	🬷	🬸	🬹	🬺	🬻 


🬆 🬀	🬥
    
🬊 🬁 🬙  

🬱 🬏 🬳  

🬵 🬞 🬶      


▓▓
▓🬆

▓▓
▓🬀

▓▓
▓🬥

▓▓
▓🭡

▓▓
▓🭠

▓▓
▓🭟

▓▓
▓🭞

▓▓
▓🭝

▓▓
▓🭜

▓▓
▓🭛

▓▓
▓🭚

▓▓
▓🭙

▓▓
▓🭘

▓▓
▓🭗

🬼	🬽	🬾	🬿 🭀
🭁	🭂	🭃	🭄	🭅	🭆

🭇	🭈	🭉	🭊	🭋
🭌	🭍	🭎	🭏 🭐	🭑

🭒	🭓	🭔	🭕	🭖
🭗	🭘	🭙	🭚	🭛	🭜

🭝	🭞	🭟 🭠	🭡
🭢	🭣	🭤	🭥	🭦	🭧

🭨	🭩	🭪	🭫	🭬	🭭	🭮	🭯


🭰	🭱	🭲	🭳	🭴	🭵	🭶	🭷	🭸	🭹	🭺	🭻	🭼	🭽	🭾	🭿

🮀	🮁	🮂	🮃	🮄	🮅	🮆

🮇	🮈	🮉	🮊	🮋

🮌	🮍	🮎	🮏

  ▓▓          
 🮍▓▓          
  ▓▓       

 🮍▒▒▒▒            
 🮍▒▒▒▒            
  ▒▒▒▒            
 🮍▒▒▒▒            
        
 🮍🮎🮏🮍🮎🮏🮍🮎🮏🮍🮎🮏🮌            
 🮍🮎🮏🮍🮎🮏🮌            
               
  🮍🮎🮏🮎🮏🮎🮏🮎🮏🮎🮏🮎🮏🮎🮏🮎🮏🮌             
  🮍🮎🮏🮌          🮍🮎🮏🮌 
  🮍🮎🮏🮌          🮍🮎🮏🮌        
  🮍🮎🮏🮌          🮍🮎🮏🮌           
  🮍🮎🮏🮌          🮍🮎🮏🮌           
  🮍🮎🮏🮌          🮍🮎🮏🮌           
  🮍🮎🮏🮎🮏🮌      🮍🮎🮏🮎🮏🮌           
                           
                           
                           
                           
   🮐🮔                        
                           
   🮑🮒🮑🮒🮑🮒🮑🮒🮑🮒🮑🮒🮑🮒🮔                        
                           
  🮕🮖🮕🮖🮕🮖🮕🮖🮕🮖        🮖🮖🮖🮖🮖🮖🮖🮖
       🮖🮖🮖🮖🮖🮖🮖🮖🮖🮖🮖                                 
       🮕🮕🮕🮕🮕🮕🮕🮕🮕🮕🮕🮕                
 🮖🮖🮖🮖 🮕🮕🮕🮕                     
                       
 🮠🮢 🮣🮢 🮥🮤                    
 🮣                      
    🮨🮩   🮪  🮫  🮬	🮭               
                         
  🮪🮬🮫🮭🮪🮬🮫🮭🮪🮬🮫🮭                       
                         
  🮫🮯🮪                       
                         
                         
🮐	🮑	🮒	🮔
🮕	🮖
🮗	🮘	🮙
🮚	🮛
🮜	🮝	🮞	🮟
🮠	🮡	🮢	🮣	🮤	🮥	🮦	🮧	🮨	🮩	🮪	🮫	🮬	🮭	🮮	🮯

🮰	🮱	🮲	🮳	🮴	🮵	🮶	🮷	🮸	🮹	🮺	🮻	🮼	🮽	🮾	🮿
🯀	🯁	🯂	🯃	🯄	🯅	🯆	🯇	🯈	🯉	🯊

◰	◱	◲	◳	
◴	◵	◶	◷	
◸	◹	◺ ◿
◻	◼	◽◾
◆ ◜	◝	◞ ◟ ◠ ◡ ◢ ◣ ◤ ◥ ● ◯                    

 ◜ ◝  ◜ ◝         
  ◯    ◯            
 ◟ ◞  ◟ ◞        
    ◤ ◥         
     ◆    ◢◣ 
    ◣ ◢   ◥◤       
             
  ◠            
  ◡           
              
 ◹◺◿◸◹◺◿◸◹◺◿◸◹◺◿◸             
 ◹◻◺◿◻◸◹◻◺◿◻◸◹◻◺◿◻◸             
              
 ◢◸◥◺◢◸◥◺◢◸◥◺◢◸◥◺             
 ◥◺◢◸◥◺◢◸◥◺◢◸◥◺◢◸             

 ◿◤◹◣◿◤◹◣◿◤◹◣◿◤◹◣             
 ◹◣◿◤◹◣◿◤◹◣◿◤◹◣◿◤             

 ◿◤◹◣◢◸◥◺◿◤◹◣◢◸◥◺             
 ◹◣◿◤◥◺◢◸◹◣◿◤◥◺◢◸             

 ◢◸◥◺◢◸◥◺◢◸◥◺◢◸◥◺             
 ◹◣◿◤◹◣◿◤◹◣◿◤◹◣◿◤             

 ◿◤◹◣◿◤◹◣◿◤◹◣◿◤◹◣             
 ◥◺◢◸◥◺◢◸◥◺◢◸◥◺◢◸             
              
 ◥◣◢◤◥◣◢◤◥◣◢◤◥◣◢◤             
 ◥◼◣◢◼◤◥◼◣◢◼◤◥◼◣◢◼◤             
              
              
                           
                           
🭆	🭇	🭈	🭉	🭊	🭋

🭑	🬼 🬽	🬾	🬿 🭀 

🭜 🭗	🭘	🭙	🭚	🭛	

🭧 🭢 🭣	🭤	🭥	🭦

🭁	🭂	🭃	🭄	🭅

🭌	🭍	🭎	🭏 🭐

🭒	🭓	🭔	🭕	🭖

🭝	🭞	🭟 🭠	🭡


🭨	🭩	🭪	🭫	🭬	🭭	🭮	🭯

------------------------
------------------------
------🭉🬾----------------
-----🭊🭁🭌🬿---------------
----🭋🭪🭮🭫🭟🭗--------------
----🭅🭪-🭨🭛---------------
--🭈🭄🭬🭭🭊🭡----------------
-🭣🭔🭪🭯🭩🭠🭗----------------
--🭤🭒🭝🭚------------------
---🭦🭛-------------------
------------------------
------------------------
------------------------
```

```
npcs
ĩ ḯ Ḯ Ỉ ỉ 
enemies
ǚ ȥ Ȫ ȫ ɀ Ṏ ṏ ṻ Ṻ ẅ Ẅ ẟ ồ Ồ ớ Ớ Ṏ ṏ ʑ ζ ѯ Ѯ Ӟ Ӱ ή Ẏ

snakes
ȥ ɀ ʑ ζ  

guys
ǚ Ǚ ṏ
Ṏ ṏ ṻ
ẅ Ẅ

bandit
Ồ

horns
ớ Ớ     

wizrd?
Ẏ
ή ἡ ἦ             


weapons
ƒ Ɩ Ɨ ƚ ƪ ƭ ƾ Ɉ ɉ Ṫ ṫ ẗ ẛ ẜ ẝ ț ɟ ɨ ɩ ɩ ɭ ɹ ɺ ʖ ɻ ɽ ɿ ʃ ʅ ʄ ʆ ʇ ʈ 
˦ ˨ ˩ Ͳ ͳ ͼ ͽ Θ Φ θ Ϯ ϯ ѳ Ѻ Ґ Ә Ѭ ϼ ṝ ḹ 

sword
ƒ Ṫ ẝ ʄ ʈ ˦ ˨ Ϯ ϯ † ተ ቶ ተ ऻ	Ť

normal swords
Ṫ Ϯ ϯ † ተ ቶ ተ Ť ʈ  --     

special swords?
ɉ ḟ ƒ 
knife
ƭ ṫ ẗ ʇ    

staff
Ɩ ƪ ẛ ɭ ſ ɽ ɼ

sickle/sythe    
ƾ ɺ ʖ ን ና        

axe
Ͳ ͳ Ґ ፕ ፐ ቸ Ƭ ϝ 
 
shield
ͼ ͽ Θ Φ θ ѳ Ѻ ʘ φ ዐ ፀ    

armour
ዠ ዦ ዥ ዧ 

items
Ʌ ṑ Ṑ Ṓ ṓ ṥ ṧ ʋ ʘ ʚ Ψ δ λ π Ю б ж э Ҝ Ҩ Ӂ φ Δ ˤ 

Ƌ ƌ Ƃ ƃ 	Ђ Һ 

ữ
Ʌ - tent
ṓṑ - jars
ʚ - guts
Ψ - candles
δ - bomb?
π - table
ж - wood bundle
Ҩ - rope

Ξ
Ѓ
Ϡ
Ỽ
ỽ
Ỿ
ỿ
ɲ
ɳ
η
 ̪
ή

features
Ħ  ṩ ȝ Ƞ Ỻ ỻ ỽ ɣ ɤ ɱ ʊ ʬ ʭ 	Π П Ш ђ ѧ Ѧ Ѫ  џ 

Ħ ỻ - gate
ɣ ɤ - pendants 
ʭ - bricks
ʬ - water/grass
Ш - bars
ѧ - fire
џ - tuning fork
ỽ - emblem
ʊ - empty pot
Ђ

ዅ ዄ
ɼŀ
፠	፡	።	፣	፤	፥	፦	፧	፨
ſǭ ṍ Ɏ

⌈ ⌉ ⌠ ⎡ ⎤ ⎧ ⎫

⎛	⎜	⎝	⎞	⎟ ⎠

    ~~~
    ⏠
           
    ⏠
    ~~~

⏡

⑁	⑂	⑃	⑄

〒

⛆

❏	❐	❑	❒
  
Containers?
፩	፪	፫	፬	፭	፮	፯
፰	፱	፲	፳	፴	፵	፶	፷	፸	፹	፺	፻	፼

×ǂǁǀƤ
ȽȾƿǬǭ

attack
X \ /

܍ ܀
፠ 
Ŧ ŧ ƍ ǫ Ǫ ǭ Ǭ ȩ ȣ Ȣ ȩ ḉ ḝ ọ Ọ ˚ ϙ Ϙ  ৭ ૦


ϙ ৭ ƍ ǫ
```

# Enemies

Snakes ʑ
slimes ǚ
spiders ẅ
bandit Ồ
goblin ớ
ghoul ή

# weapons

## swords

```
Ṫ Ϯ ϯ † ተ ቶ ተ Ť
```

**longsword**

- bronze, iron, steel, titanium
  `Ṫ † ϯ`

claymore
`Ṫ`

longsword
`†`

greatsword
`ϯ`

**shortsword**

- bronze, iron, steel, titanium
  `Ϯ ተ`

shortsword
`Ϯ`

**staffs**

- pine, maple, oak
  `Ɩ ƪ ẛ ɭ ſ`

staff
`ɭ`

wood staff
`ſ`

gem topped staff
`ƪ`

druid staff
`ẛ`

**axes**

- bronze, iron, steel, titanium
  `Ͳ ͳ ፕ ፐ ቸ Ƭ`

heavy axe
`Ͳ`

light axe
`ͳ`

pick axe
`ፐ`

pickhammer
`Ƭ`

XXXX axe
`ፕ`

waraxe
`ቸ`

# armour

`ዠ ዣ ዦ ዥ ዧ`

light armour
`ዣ ዦ`

medium armour
`ዠ ዡ`

heavy armour
`ዥ ዧ`

# Shields

`Θ θ ѳ Ѻ ѻ ʘ ዐ ø Ø Ф
Ɵ ౷ ۞`

small wooden shield
`ѳ`

large wooden shield
`θ`

iron shield
`Θ`

steel shield
`ʘ`

other
`Ѻ ዐ ø Ø Ф`

**animations**:

- Fight enemy design
- Equip icons

# Features

added map elements that are interacted with similar to settlements
may rarely have one npc or a few items

**types**:

- fields, grass features
- stream
- pond/lake
- ruin
- shack
- abandoned town/farm

# Puzzles

## Maze

- use carve algo to create map X
- place item on one side X
- remove outer wall on other side X

## Teleport

- generate node graph of steps
- use graph to chain the teleport cells
- place walls between connection points

## Inverted

- use carve to make maze
- set some

# Compass

- fix menu
- switch heading
- scroll list

# Notebook

## Settlements

- settlements that are found are added to this panel
- profile is populated with content from generated settlement
- Location relative to origin, closest settlement
- Shops that are present:
  - Shop names
  - ShopNPC names
- resident NPC names
- population
- possible comment about feature?

## Conversations

- conversations had with CommNPC, and ConvNPC entities are saved with their name

```
CommNPC
--

  Kevin: "This is a thing that the player said."

--
```

```
ConvNPC
--

  Kevin: "This is a thing that the NPC said."

  You: "This is the response the player chose."

  Kevin: "This is the thing that the NPC said in response to your choice."

  You: "This is the next response the player picked."

--
```

## Knowledge

- information about the caves and general lore is put here
- provided as entries with headers

```
  This is a header

  This is an entry paraphrasing the knowledge gained by an encounter.
  The length of an entry should be informative and concise.

  - The entry can also include a list
  - The list should be used for specific details
  - The list should be at most around 5-7
```

## Discoveries

- entries for spawned puzzles are provided here.
- when the player talks to a SpawnNPC the entry is added
- the entry contains a description of the spawned location
- the entry should be a paraphrasing of the content of the SpawnNPC conversation

```
This is a description of the spawned location that contains the
same information that was provided in the conversation with the
SpawnNPC. This contains a general description of the location
and the position relative to the current location, possibly
the position relative to the closest settlement
```

# Equipment

## Weapons

Add buff to damage
Some add buff to attack

## Armour

Add buff to defence
Some add buff to attack

## Shield

Add buff to defence

## Enchanted Items

Provide buffs and resistances

items such as:

- amulets
- rings
- random objects

## Cursed items

some enchanted items are cursed and have negative effects in addition to their positive ones

eg: +10 to damage, but -5 to defence

# Experience

Can be accumulated to increase capabilities

## Fighting stats

- Attack: gain xp when attack roll hits
- Defence: gain xp when enemy attack roll misses
- Damage: gain xp when damage roll is > max/2

# Non-combat stats

- Trading: The more a character trades with NPCs, the better deals the player gets
- Herbalism: The player can learn more about the various fungi in the caves
  - can be used to create Health Potions, Salves, Anti-toxin, other buff potions (Agility Potion)
  - can be developed by finding fungi in the caves
  - in the caves//settlements there are herbalists that will identify the fungi you find
  - they will also tell you what you can make with the various fungi types
- Medicine: The player can develop the ability to heal themselves
  - the player can talk to some of the cliic managers and receive first aid education
  - as the player visits different clinics they can upgrade their abilities to heal themselves
- Luck: Increases the lkelyhood of having better drops, improves luck on rolls
- Navigation: Improves ability to move through the caves, and know where places are.

## Lockpicking

- some doors have locks
- doors have security levels 0-31
- player earns xp for success
- player must start with easier doors, etc.

door locations:

- features
- settlements
- puzzles?

## Luck

- gain xp by having winning rolls
- luck potions are rare loot
- pray at cave temple
- gather `x` for cave cultists

## Navigation

- gain xp by finding locations: {settlements, puzzles, features}

Levels:

1. basic directional compass to closest settlement - maybe puzzle
2. history
3. distance gauge
4. xy distance
5. small map
6. large map

## Herbalist

- player can find plants/fungi in the caves
- settlements have herbalists that can identify plant that the player brings them
- known plants can be used to make potions and other items

**Luminous Mushroom**:

`ų`

- have a low-med probability of being spawned
- can be used to make health potions

**Moss**:

- have a med probability of being spawned
- can be used to make salves

**Lichenous Growth**:

- have a med probability of being spawned
- can be used to make xxx potions

**Lucky clover**:

- have a med probability of being spawned
- can be used to make xxx potions

**Shroom**:

- causes player to enter altered state
  - they do not feel pain and are bold when attacking
  - the damage will be felt when the effect wears out
- takes 25% damage
- +5 to attack
- the outstanding damage is accumulated in a var, and is applied at once when the effect ends
- if the amount is more than the player's max health, this will cause instadeath
- if the player is at their max health, it is reduced to half
- this the only way to survive an accumulation higher than max health

## Trading

- Each time the player trades with a TradeNPC they gain more xp
- The higher the xp, the better deals they get when buying and selling
- This can be used to increasingly gain wealth throughought the game.

## plants

- plants that can be used for herbalism are placed
- there is roll done to determine which plants are generated.
- place the plants similar to items, but in groupings
- there could be roll for plant being overgrown, causes plant to fill empty space.
