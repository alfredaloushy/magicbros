import sys, math, shutil
from pathlib import Path
sys.path.insert(0,'/tmp/wild-magic-doc-tools')
from PIL import Image, ImageDraw, ImageFont
from docx import Document
from docx.shared import Inches, Pt, RGBColor
from docx.oxml import OxmlElement
from docx.oxml.ns import qn
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image as RLImage, Table, TableStyle, PageBreak, KeepTogether
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.utils import ImageReader
from xml.sax.saxutils import escape
ROOT=Path('/home/abe/minidupe/documents'); BUILD=ROOT/'.build'
INK='#202641'; TEAL='#087F8C'; GOLD='#D79335'; MUTED='#65718A'; LIGHT='#EDF3F6'

def font(n): return ImageFont.load_default(size=n)
def canvas(name,title):
    im=Image.new('RGB',(1440,760),'#F4F6FB'); d=ImageDraw.Draw(im); d.text((40,24),title,font=font(35),fill=INK); return im,d
def box(d,xy,text,fill='#FFFFFF',size=25):
    d.rounded_rectangle(xy,18,fill=fill,outline='#C5D2DF',width=3)
    x,y,x2,y2=xy; lines=text.split('\n'); lineh=size+9
    for j,line in enumerate(lines): d.text(((x+x2)/2,(y+y2)/2+(j-(len(lines)-1)/2)*lineh),line,font=font(size),fill=INK,anchor='mm')
def arrow(d,p,q,color=TEAL):
    d.line([p,q],fill=color,width=7); a=math.atan2(q[1]-p[1],q[0]-p[0]); end=[q,(q[0]-22*math.cos(a-.5),q[1]-22*math.sin(a-.5)),(q[0]-22*math.cos(a+.5),q[1]-22*math.sin(a+.5))]; d.polygon(end,fill=color)
def save(im,name):
    p=BUILD/(name+'.png'); im.save(p); return str(p)

im,d=canvas('loop','THE MATCH LOOP / first private-room release')
labels=['Private room\nChoose signature','Ready up\nEnter courtyard','Fight + explore\nFind second spell','Combine spells\nSpend mana','Defeat / respawn\nKeep signature','Timer expires\nMost kills wins']
for i,s in enumerate(labels):
    row=i//3; col=i%3 if row==0 else 2-i%3; x=50+col*460;y=140+row*270
    box(d,(x,y,x+400,y+170),s,fill='#FFFFFF' if i!=3 else '#D9F4F0')
arrow(d,(450,225),(505,225));arrow(d,(910,225),(965,225));arrow(d,(1170,310),(1170,395));arrow(d,(965,495),(910,495));arrow(d,(505,495),(450,495))
d.text((65,680),'Tied leaders only: sudden death, no respawn, last survivor wins.',font=font(26),fill=MUTED)
LOOP=save(im,'match-loop')

im,d=canvas('controls','MOVEMENT + CONTROLS / conceptual layout, not a final HUD')
d.rounded_rectangle((100,110,1340,650),36,fill='#DDE8F0',outline=INK,width=5)
d.rectangle((170,410,625,435),fill='#62768B'); d.rectangle((795,310,1250,335),fill='#62768B')
d.ellipse((650,280,698,328),fill='#9B56BC');d.polygon([(640,283),(672,235),(707,283)],fill='#9B56BC');d.rectangle((651,324,695,377),fill='#9B56BC')
arrow(d,(700,330),(865,225));d.text((410,205),'Run > double jump > air dash',font=font(25),fill=INK)
for xy,col in [((170,455,320,605),'#A2B8CA'),((1100,455,1250,605),'#99D9E0')]:d.ellipse(xy,fill=col,outline=INK,width=3)
d.text((245,530),'MOVE',font=font(25),fill=INK,anchor='mm');d.text((1175,530),'AIM',font=font(25),fill=INK,anchor='mm')
box(d,(810,510,1020,590),'Spell switch',size=22);box(d,(810,410,1020,485),'Jump / dash*',size=22)
d.text((145,145),'Health + mana',font=font(25),fill=INK);d.text((1010,145),'Timer / score',font=font(25),fill=INK)
d.text((45,690),'*Jump / dash button arrangement needs a phone usability test. Prototype: quick casts only.',font=font(25),fill=MUTED)
CONTROLS=save(im,'controls')

im,d=canvas('fusion','FUSION CAST / later release behavior')
steps=[('01  ARM','Hold switch\nCheck mana + cooldown'),('02  AIM','Aim / release to commit\nBrief wind-up begins'),('03  WIND-UP','Slow movement; aim freely\nNo jump or dash'),('04  FIRE','Aim / placement locks\nEffect activates')]
for i,(a,b) in enumerate(steps):
    x=40+i*350;box(d,(x,170,x+310,410),a+'\n\n'+b,fill='#D9F4F0' if i==2 else '#FFFFFF',size=23)
    if i<3:arrow(d,(x+310,290),(x+345,290))
box(d,(60,485,680,645),'Damage / knockback: casting continues\nDeath: casting stops',size=27)
box(d,(760,485,1380,645),'Each fusion has its own cooldown\nSwapping spells cannot reset it',size=27)
d.text((50,700),'Mana debit, cooldown start and death refunds: implementation rules still to finalize.',font=font(25),fill=MUTED)
FUSION=save(im,'fusion-sequence')

im,d=canvas('arena','TRAINING COURTYARD / functional side-view schematic')
for xy in [(70,300,540,335),(900,300,1370,335),(160,565,540,600),(900,565,1280,600),(220,150,460,182),(980,150,1220,182),(570,200,670,230),(770,200,870,230)]:d.rounded_rectangle(xy,8,fill='#6C7B8D')
d.rectangle((540,300,900,326),fill='#BA854E');d.text((720,355),'BREAKABLE BRIDGE',font=font(24),fill=INK,anchor='mm')
d.polygon([(540,420),(900,420),(840,650),(600,650)],fill='#423254'); d.text((720,545),'LETHAL PIT',font=font(26),fill='#FFFFFF',anchor='mm')
for x in [340,1100]: d.ellipse((x-24,101,x+24,149),fill='#E45D70');d.text((x,80),'HEALTH',font=font(22),fill=INK,anchor='mm')
for x in [350,1090]: d.ellipse((x-24,513,x+24,561),fill='#48B3DC');d.text((x,490),'MANA',font=font(22),fill=INK,anchor='mm')
for x,y in [(155,265),(720,164),(1285,265)]:d.polygon([(x,y-20),(x+20,y),(x,y+20),(x-20,y)],fill='#A26BDA')
arrow(d,(400,470),(455,360));arrow(d,(1010,355),(1100,465));arrow(d,(490,240),(585,175));arrow(d,(880,175),(965,240))
d.text((40,650),'Outer edges are lethal. Permanent floating platforms bypass a missing bridge.',font=font(25),fill=INK)
d.text((40,700),'Purple diamonds = three spell spawns. Symmetrical routes; distinct scenery. Not to scale.',font=font(24),fill=MUTED)
ARENA=save(im,'arena-map')

im,d=canvas('pairs','FOUR BASE SPELLS / SIX TWO-ELEMENT FUSIONS')
pairs=[('FIRE + GUST','Flame Tornado','Moving pull + lift'),('FIRE + STONE','Magma Bomb','Arc > blast > burning ground'),('FIRE + FROST','Steam Burst','Blast + shared obscuring steam'),('GUST + STONE','Orbiting Stones','Defend > launch'),('GUST + FROST','Ice Jet','Dash through enemies'),('STONE + FROST','Glacier Wall','Aim > place > breakable cover')]
for i,(a,b,c) in enumerate(pairs):
    x=40+(i%2)*710;y=110+(i//2)*200;box(d,(x,y,x+650,y+170),a+'\n'+b+'\n'+c,fill=['#FFF0DC','#FFF0DC','#FFF0DC','#E5F4F7','#E5F4F7','#EAF0FF'][i],size=25)
PAIRS=save(im,'fusion-pairs')

pages=[]
def page(title,subtitle=''): p={'title':title,'subtitle':subtitle,'blocks':[]};pages.append(p);return p
def para(p,t):p['blocks'].append(('p',t))
def sub(p,t):p['blocks'].append(('h',t))
def bullets(p,*items):
    for t in items:p['blocks'].append(('b',t))
def table(p,headers,rows,widths=None):p['blocks'].append(('t',(headers,rows,widths)))
def fig(p,path,caption):p['blocks'].append(('i',(path,caption)))

p=page('WILD MAGIC ACADEMY','Game concept & two-player prototype plan | Draft 1.0 | 14 November 2026')
para(p,'A fast, mischievous 2D magic arena brawler for friends. Leap between platforms, turn an incoming projectile against its caster, and combine familiar spells into spectacular new threats.')
fig(p,str(BUILD/'concept-art.png'),'Generated exploratory concept art: a storybook courtyard with pixel characters, four base spells and six fusion studies. This establishes a possible direction, not final assets or map geometry.')
table(p,['DESIGN NORTH STAR','IMMEDIATE BUILD'],[['4–6 friends; private rooms; Android first.','Two players on local Wi-Fi.'],['Readable characters inside wild magical spectacle.','Movement, quick-cast Fireball + Gust.'],['Skill, experimentation and controlled chaos.','Simple arena and automatic respawns.']],[.5,.5])
para(p,'Working title. The concept takes inspiration from Mini Militia’s compact side-view combat and accessible multiplayer feel, with an original magical school setting and spell system.')

p=page('01 / The experience','Confirmed concept and design boundaries')
sub(p,'The fantasy')
para(p,'Students settle academy rivalries through chaotic courtyard duels. Characters are playful and expressive; spell behavior is consistent enough to learn, while combinations, terrain and knockback create surprising encounters.')
sub(p,'Three design pillars')
bullets(p,'Accessible action: simple mobile inputs, fast movement and quick returns to combat.','Skillful experimentation: aiming, timing, positioning, projectile redirection and spell combinations reward practice.','Controlled spectacle: dramatic magic and comic reactions, with clear character silhouettes and predictable attack behavior.')
table(p,['AREA','FIRST PRIVATE-ROOM RELEASE','LATER'],[['Players / mode','2–6 supported; designed for 4–6; free-for-all.','Other modes are not yet specified.'],['Platform','Android.','iOS and PC.'],['Networking','Private rooms over local Wi-Fi; host device runs room.','Online rooms and host migration.'],['Appearance','One shared wizard silhouette; unique player colors.','Magical species with equal gameplay capabilities.'],['Art','Provisional storybook + pixel art blend.','Revise after an in-motion style test if needed.']],[.2,.47,.33])
sub(p,'How to read this draft')
para(p,'Confirmed choices are the agreed design direction. Prototype requirements describe only the immediate build. Suggestions and unresolved rules are explicitly labeled; numerical balance remains provisional until tested. Visual schematics explain relationships rather than final dimensions or button positions.')

p=page('02 / The match loop','Short matches, repeated encounters, most kills wins')
fig(p,LOOP,'A normal match returns players to the lobby after results. The prototype uses an even smaller loop: spawn, fight, defeat, respawn.')
bullets(p,'Host selects arena, duration (3, 5 or 10 minutes), player limit and room lock. Players select signature spells and ready up.','Explore for a second spell, mana and health. Fight using movement, base spells and later fusions.','Defeat returns a player after roughly two seconds. Keep the signature spell; drop the arena spell so someone else can collect it.','At time expiration, highest kill count wins. Results show rank, kills, deaths and playful awards; return to the lobby to change spells and ready up again.')
sub(p,'Tie resolution')
para(p,'All tied leaders enter sudden death. Everyone else is instantly removed from combat and spectates. Leaders retain their current health, mana, spells and positions; healing and respawns are disabled. After a delay, a closing magical boundary prevents indefinite stalling. Last survivor wins.')

p=page('03 / Movement & mobile controls','Fast, springy movement with frequent landings')
fig(p,CONTROLS,'Control intent is confirmed; the exact jump/dash arrangement and casting gesture need a phone usability test.')
table(p,['ACTION','BEHAVIOR'],[['Movement','Run, double jump and directional air dash. Basic movement is free.'],['Basic casting','Right-side aim control: aim, hold to charge and release to cast. First prototype uses quick casts only.'],['Spell selection','Tap the switch control to swap between equipped spells.'],['Fusion input','Hold switch to arm the fusion, then aim and release to commit. If unavailable, keep the active spell and show why.'],['Collection','Health and mana collect automatically. Tap a nearby pickup control to replace the arena spell.'],['Camera','Follow the local character at fixed zoom. Show edge arrows only for nearby off-screen opponents.']],[.24,.76])
para(p,'Extended magical movement such as hovering or flight was reserved as mana-costing behavior; whether to include it, and how it controls, remains unresolved. It is outside the first prototype.')

p=page('04 / Combat rules & resources','Encounters reward aim, positioning and recovery')
sub(p,'Damage, health and mana')
bullets(p,'Target approximately 4–6 ordinary hits per defeat. This is an encounter durability goal, not a fixed spell damage table.','Health regenerates partially through passive recovery; health pickups restore the remainder. Exact thresholds, delays and amounts need testing.','Mana regenerates naturally and can be restored through pickups. Fusion attacks consume mana and require a cooldown.','Strong knockback belongs mainly to heavy or control attacks. Avoid making every hit equally disruptive.','A caster takes reduced damage from their own damaging spells. Their surface effects and recoil still affect them; slippery ground affects everyone.')
sub(p,'Defeat and credit')
bullets(p,'Use safe spawn selection and brief spawn protection that ends when the player attacks.','Falls into the central pit or off lethal arena edges cause defeat. Credit the last attacker if the fall occurs within a short credit window.','Accidental falls and self-caused deaths do not subtract score. Exact credit window is a tuning parameter.','Prototype exception: respawn restores health and both test spells; signature/pickup loss is introduced later.')
sub(p,'Combat information')
para(p,'Opponent health bars appear briefly after damage, then fade. Opponent loadouts remain hidden until their spells are cast; do not add persistent active-spell glows or loadout symbols. Player color, name and nearby edge indicators make individuals trackable.')
sub(p,'Open edge cases')
para(p,'Simultaneous final deaths in sudden death, source attribution for reflected projectiles, and collection behavior at full health/mana need explicit rules before implementing those systems. Do not let these block the quick-cast prototype.')

p=page('05 / Four base spells','Equally viable signature choices; different reach, timing and utility')
table(p,['SPELL','QUICK CAST','CHARGED CAST','OVERCAST'],[['Fireball','Projectile explodes on first enemy or surface.','Larger blast.','Strong backward recoil.'],['Gust','Pushes enemies; returns incoming projectiles toward their original caster.','Redirects projectiles along your aim.','Backward launch / recoil on caster.'],['Stone Fist','Short-range magical punch.','Greater reach and knockback.','Pillar rises beneath caster, lifting them and striking nearby enemies.'],['Frost Bolt','Fast projectile slows enemies.','Creates slippery ground affecting all players, including caster.','Distinct overcast consequence not yet specified.']],[.16,.29,.28,.27])
sub(p,'Charge behavior')
para(p,'Hold to increase power, with predictable side effects increasing at overcast thresholds. Maximum charge auto-fires. Clear animation and sound communicate charge stages. There is no accumulated instability meter; consequences belong to the individual cast. Provide a lobby spell preview.')
sub(p,'Signature + arena spell')
bullets(p,'Choose one specific signature before a match. All four spells can be signatures and should offer similar overall combat potential without identical damage numbers.','Carry one replaceable arena spell alongside the signature. A duplicate signature pickup restores mana rather than replacing the second slot.','Several players may choose the same signature. Base spells remain useful when a fusion is unavailable.','Prototype exception: everyone starts with Fireball and Gust, switches freely and uses quick casts only. No charging, overcasting or arena spell collection yet.')
para(p,'Exact attack rates, projectile sizes, damage, mana use and charge timing are deliberately unassigned. Establish them through play, beginning with Fireball and Gust.')

p=page('06 / The fusion system','Six distinct pairings; combination order does not matter initially')
fig(p,PAIRS,'Each pairing has a fixed identity. The first release has no same-element fusions and no fusion overcasting.')
bullets(p,'A fusion uses the two equipped elements. Signature/pickup order has no effect initially; order-dependent combinations are a later addition.','Every fusion has its own mana cost and cooldown, based on power and utility. Exact values require testing.','Cooldown belongs to the player. Swapping the arena spell must not reset, shorten or bypass an active fusion cooldown. A dropped spell carries no previous owner cooldown.','Unresolved implementation detail: define the cooldown data model and its behavior across every pair transition. Confirm the above anti-reset rule before adding spell pickups.')

p=page('07 / How fusions behave','Consistent rules create counterplay')
table(p,['FUSION','BEHAVIOR & COUNTERPLAY'],[['Flame Tornado | Fire + Gust','Moving damaging vortex pulls and lifts enemies. Affected players retain casting and movement, including dash escape; this is not a hard capture.'],['Magma Bomb | Fire + Stone','Heavy arcing projectile produces an explosion and temporary burning ground. Its trajectory and persistent hazard shape where players move.'],['Steam Burst | Fire + Frost','Area damage and knockback followed by obscuring steam. Steam affects everyone’s visibility, including the caster.'],['Orbiting Stones | Gust + Stone','Temporary, limited projectile defense. Repeat the fusion input to aim and launch stones early. Surviving stones automatically launch along the current aim when the timer expires.'],['Ice Jet | Gust + Frost','Directional dash damages each enemy at most once, passes through enemies and leaves a slippery trail. Terrain stops the dash.'],['Glacier Wall | Stone + Frost','Aim a placement preview within limited range; release to place a temporary, breakable barrier. Exact valid-placement and blocked-placement rules still need definition.']],[.29,.71])
sub(p,'Shared casting commitment')
para(p,'Every fusion has a brief, recognizable animation and sound. During the wind-up, the caster moves slowly and can adjust aim, but cannot jump or dash. Direction or placement locks when the effect fires. Damage and knockback do not cancel the cast; death does. Fusions begin at fixed power without overcasting.')
para(p,'Special interactions still need testing: reflected projectile ownership and damage credit, Glacier Wall placement through occupied spaces, overlapping area effects and interactions between temporary structures. Do not assume unspecified interactions are already settled.')

p=page('08 / Casting in a real fight','A visual sequence for input, commitment and release')
fig(p,FUSION,'The fusion wind-up gives opponents a chance to read the attack while keeping aiming responsive.')
sub(p,'Example: Fireball + Gust')
para(p,'A player with Fireball and Gust arms Flame Tornado. If mana or cooldown prevents it, the game explains the block and leaves the active spell unchanged. Otherwise the player aims and commits. A distinct wind-up begins: they can move slowly and track a target. An incoming hit pushes them but does not cancel the cast. On release from the wind-up, the tornado follows the final aim. Opponents caught in it can still cast and dash out.')
sub(p,'Implementation decisions to make before fusion development')
bullets(p,'When exactly mana is spent and cooldown starts; whether death during wind-up refunds anything.','How Gust interacts with each projectile type and what happens when a projectile is redirected several times.','How the casting slowdown interacts with existing airborne velocity and incoming knockback.','How to make the switch/arm gesture distinct from an ordinary switch on a small touch screen.')

p=page('09 / Training Courtyard','First arena: connected loops, three levels and player-created danger')
fig(p,ARENA,'Layout intent only. Colored circles represent separate supply locations, not final pickup art. Exact dimensions and reachability require movement tests.')
bullets(p,'Lower routes provide sheltered recovery and mana. Main-level spaces hold most fights. Upper platforms expose players in exchange for health access.','Mostly symmetrical main routes support fairness, while landmarks, scenery and small variations help orientation.','A breakable wooden bridge crosses the central lethal pit. Permanent spaced platforms above it preserve an alternative route.','The bridge rebuilds after a fixed delay with a visible warning before becoming solid. Other selected breakables can also return; most terrain stays fixed.','Three evenly spread spell spawns share the same respawn interval and roll random spell contents. Health and mana occupy separate locations.','Initial danger is player-created: spells, structures, bridge destruction and knockback near lethal edges. Add one predictable environmental hazard later.')

p=page('10 / Private rooms & connection behavior','Local Wi-Fi first; Android first')
sub(p,'Room flow')
para(p,'Discover nearby private rooms, with a manual local-address fallback. A host creates a room and selects arena, duration, player limit and room lock. Players join, customize, choose a signature and ready up. After a match, everyone returns to the lobby. No public matchmaking is required for this scope.')
table(p,['SITUATION','AGREED BEHAVIOR'],[['Host device','Hosts the initial version. Host departure or disconnection ends the match; host migration is deferred.'],['Late join','Spectate until the next match.'],['Temporary disconnect','A grace period preserves score. A reconnecting player returns through a respawn. Exact grace duration is open.'],['Local play without internet','Aim to support a hotspot or local Wi-Fi without internet where Android devices permit it. Validate on real phones.'],['Future networking','Online support and host migration come later. Android comes first; iOS and PC follow.']],[.27,.73])
sub(p,'Prototype networking boundary')
para(p,'Start with two real Android devices on the same network. A simple create/join path is sufficient for the first test; nearby discovery is a first-release requirement. Verify movement, collision, damage, projectile redirection and respawn agreement between the devices before adding more players.')
sub(p,'Engineering recommendation, not a previously selected engine')
para(p,'Use one authoritative host to resolve shared combat outcomes and keep the client view responsive. The engine, transport, synchronization approach, update rate and device baseline still need selection. This draft commits to observable behavior rather than a particular stack.')

p=page('11 / Art, sound & readability','Wild magic that remains visible and affordable to render')
sub(p,'Provisional visual direction')
para(p,'Blend storybook charm with pixel art: expressive pixel characters and gameplay terrain, illustrated academy backgrounds, whimsical architecture and textured color. Treat this as a reversible direction. Test it in motion before producing a full asset set; check that pixel density, scaling and background detail feel coherent on phones.')
sub(p,'Character identity and customization')
bullets(p,'Begin with one readable wizard silhouette. Use unique room colors consistently across outfit accents, names and indicators. Players choose hats and accessories.','Make the starter cosmetic selection available immediately. Later, add new achievement cosmetics while retaining starter access.','Add magical species later, with equal gameplay capabilities. Visual differences must not obscure hitboxes or change competitive reach.')
sub(p,'Spectacle budget')
para(p,'Use dramatic bursts, vivid elemental motion and brief screen effects. Preserve character silhouettes above effects, keep lingering particles limited and scale graphics quality for weaker phones. Steam remains a deliberate gameplay visibility effect; quality settings should preserve its tactical meaning. Exact particle counts, frame-rate target and minimum hardware are not yet set.')
sub(p,'Sound and physical feedback')
para(p,'Use playful magical mayhem: punchy spells, comic impacts, mischievous music and distinct danger cues. Heavy hits and big fusions get brief shake and vibration; ordinary attacks get light feedback. Offer separate intensity controls. Characters act primarily through animation, with occasional short kill or fusion-hit quips and a chatter cooldown.')

p=page('12 / Scope boundaries','Keep the immediate plan small while preserving the wider concept')
table(p,['IMMEDIATE PROTOTYPE','FIRST PRIVATE-ROOM RELEASE','LATER / FINAL VERSION'],[['Two players over local Wi-Fi.','2–6 supported, tuned for 4–6; private rooms.','Online, host migration, iOS and PC.'],['Simple functional test arena.','Training Courtyard with three levels, supplies and breakable bridge.','A predictable environmental hazard; other arenas unspecified.'],['Fireball + Gust, both equipped; quick casts only.','Four base spells, signature + pickup system, charging and overcasting; six fixed-power fusions.','Order-dependent fusions and fusion overcasting.'],['Fast movement, aim, switch and automatic respawn.','Full health/mana rules, timed scoring, tie resolution and results.','Detailed combat statistics.'],['Basic temporary visuals sufficient for testing.','Shared wizard customization; starter items available; playful awards.','Magical species and achievement cosmetics.'],['No tutorial required for the first test.','Optional solo practice courtyard.','Guided tutorial for the final version.']],[.30,.37,.33])
para(p,'This is a scope map, not a committed release schedule. The next development plan covers the immediate prototype only. Nothing in the concept requires implementing every future feature before testing with two players.')
sub(p,'Prototype exclusions')
para(p,'Exclude fusions, charging, overcasting, full room discovery, cosmetic progression, detailed statistics, the finished courtyard, pickup-based loadouts and complete match scoring from the first build. Prove movement and combat first; add these systems only after the agreed checklist passes.')

p=page('13 / Two-player prototype plan','Detailed implementation order; no unverified time estimates')
table(p,['STEP','BUILD','EVIDENCE TO ADVANCE'],[['1. Local movement','Simple arena; run, double jump, directional air dash; basic collisions and camera.','A player can traverse and repeatedly land on intended routes without fighting controls.'],['2. Touch combat','Aim/release quick casts; switch between Fireball and Gust; clearly readable hit feedback.','Both spells can be aimed and switched reliably on a phone.'],['3. Local connection','One Android device creates a session; a second joins over the same Wi-Fi. Mirror players and projectiles.','Two physical phones remain connected while moving and casting.'],['4. Combat agreement','Shared health and collision results; Fireball explosion; Gust enemy push and projectile return.','Both devices agree on hits, direction changes and defeats.'],['5. Respawn loop','Short defeat delay; restore health and both equipped spells; safe starting positions.','Repeated defeats return both players to combat without duplicated actors or stale projectiles.'],['6. Playtest and revise','Run the agreed checklist; tune movement, aim, blast and knockback. Keep parameter changes easy.','Both players find the core action responsive and can intentionally use its mechanics.']],[.16,.45,.39])
sub(p,'Prototype spell behavior')
para(p,'Fireball explodes at the first enemy or surface. Gust pushes enemies and returns incoming projectiles toward their original caster. Both players have both spells from spawn and retain them after defeat. There are no fusion inputs, charge states or overcast recoil in this milestone.')
para(p,'Recommendation: keep a lightweight visible connection/debug indicator during development, and inspect gameplay events on both devices when investigating disagreement. This is a development aid, not a requirement for the final player interface.')

p=page('14 / Playtest gate & remaining decisions','The prototype succeeds by demonstrated feel and reliability')
sub(p,'Agreed pass checklist')
bullets(p,'Both players move responsively over local Wi-Fi and can run, double jump, air dash and land intentionally.','Both can aim and cast Fireball, judge its impact and dodge incoming attacks.','Both can switch spells reliably without triggering unwanted actions.','Gust reliably returns projectiles toward the original caster and provides understandable enemy push.','Both devices agree on major combat outcomes, and automatic respawns consistently restore play.','Movement and combat feel good enough to justify expanding the game. Fix unclear or unreliable actions before adding fusions.')
sub(p,'Practical test session — suggested procedure')
para(p,'First traverse the arena without attacking. Next trade stationary Fireballs and attempt Gust returns. Then repeat while moving and dodging. Finally play several uninterrupted fight/respawn cycles. Record reproducible failures separately from balance preferences. Test at least one hotspot/no-internet arrangement before treating it as supported.')
sub(p,'Decisions reserved for development and playtesting')
para(p,'Select engine and network transport; establish supported devices and performance targets; finalize touch gestures; tune damage, movement, mana and timings; define reflected kill ownership, fusion cooldown transitions and casting resource debit; resolve sudden-death simultaneous defeats and wall placement validity. Frost Bolt’s overcast effect and extended magical movement are also open.')
sub(p,'After this milestone')
para(p,'Review the checklist together, then choose the next build goal. A sensible candidate is charging and a first fusion, but that next scope has not been selected. Preserve the full concept as a reference without allowing it to expand the two-player prototype prematurely.')

p=page('15 / Visual reference & provenance','Editable document and PDF share this source content')
sub(p,'What the visuals communicate')
para(p,'The generated concept plate explores environment mood, wizard silhouettes, base spell identity and fusion spectacle. The match loop, controls sketch, fusion pairing chart, casting sequence and arena schematic explain confirmed mechanics. Drawn geometry is explanatory; the arena is not to scale and the HUD is not a final touch layout.')
sub(p,'Concept-art prompt')
para(p,'Built-in image generation; one generated concept plate. Prompt specification: polished landscape concept sheet for an original 2D Android magical academy arena brawler; storybook illustrated backgrounds with crisp pixel wizard characters, terrain edges and spells; three-level courtyard with floating platforms, central wooden bridge and lethal pit; four readable wizards with distinct outfit accents; spectacular but silhouette-preserving Fireball and Gust effects; lower studies showing wizard outfits, four base spells and all six fusions; no text, UI, logos or watermark.')
sub(p,'Production guardrails')
bullets(p,'This artwork is exploratory concept art, not a finished sprite sheet, animation set or performance-tested game scene.','Keep gameplay rules authoritative when concept art simplifies geometry or spell behavior. Use the labeled diagrams for mechanical relationships.','The storybook/pixel mixture is provisional. Validate it in an animated phone mockup and revise if the combination does not work.','The document does not select an engine, promise dates, assign final tuning values or imply that a playable build already exists.')
sub(p,'Document package')
para(p,'Wild_Magic_Academy_Design_Draft.docx is the editable master. Wild_Magic_Academy_Design_Draft.pdf is the shareable reading copy. Both embed the concept art and the same diagrams. Supporting build assets are retained locally in the hidden .build directory.')

# Word output
D=Document();sec=D.sections[0];sec.page_width=Inches(8.27);sec.page_height=Inches(11.69);sec.top_margin=Inches(.65);sec.bottom_margin=Inches(.65);sec.left_margin=Inches(.70);sec.right_margin=Inches(.70)
normal=D.styles['Normal'];normal.font.name='Calibri';normal.font.size=Pt(10);normal.paragraph_format.space_after=Pt(7)
for name,size,col in [('Title',28,INK),('Heading 1',22,INK),('Heading 2',13,TEAL)]:
    s=D.styles[name];s.font.name='Calibri';s.font.size=Pt(size);s.font.color.rgb=RGBColor.from_string(col[1:]);s.paragraph_format.space_after=Pt(8)
header=sec.header.paragraphs[0];header.text='WILD MAGIC ACADEMY  /  DESIGN DRAFT 1.0';header.style='Caption'
foot=sec.footer.paragraphs[0];foot.text='PRIVATE-ROOM MAGIC BRAWLER  •  ';fld=OxmlElement('w:fldSimple');fld.set(qn('w:instr'),'PAGE');foot._p.append(fld)
for idx,p in enumerate(pages):
    if idx:D.add_page_break()
    D.add_heading(p['title'],0 if idx==0 else 1)
    x=D.add_paragraph(p['subtitle']);x.style='Subtitle';x.runs[0].font.size=Pt(11)
    for kind,data in p['blocks']:
        if kind=='p':D.add_paragraph(data)
        elif kind=='b':D.add_paragraph(data,style='List Bullet')
        elif kind=='h':D.add_heading(data,2)
        elif kind=='i':
            path,cap=data;D.add_picture(path,width=Inches(6.8));D.add_paragraph(cap,style='Caption')
        elif kind=='t':
            heads,rows,widths=data;t=D.add_table(rows=1, cols=len(heads));t.style='Light Shading Accent 1'
            for j,h in enumerate(heads):t.rows[0].cells[j].text=h
            for row in rows:
                cells=t.add_row().cells
                for j,cell in enumerate(row):cells[j].text=cell
            for row in t.rows:
                for cell in row.cells:
                    for par in cell.paragraphs:
                        par.paragraph_format.space_after=Pt(5)
                        for run in par.runs:run.font.size=Pt(9)
            for cell in t.rows[0].cells:
                for r in cell.paragraphs[0].runs:r.bold=True
            if widths:
                t.autofit=False
                for row in t.rows:
                    for j,cell in enumerate(row.cells):cell.width=Inches(6.8*widths[j])
D.save(ROOT/'Wild_Magic_Academy_Design_Draft.docx')
# PDF output
styles=getSampleStyleSheet()
styles.add(ParagraphStyle(name='BodyWM',fontName='Helvetica',fontSize=9.7,leading=14,textColor=colors.HexColor(INK),spaceAfter=8))
styles.add(ParagraphStyle(name='TitleWM',fontName='Helvetica-Bold',fontSize=27,leading=31,textColor=colors.HexColor(INK),spaceAfter=12))
styles.add(ParagraphStyle(name='HeadWM',fontName='Helvetica-Bold',fontSize=21,leading=25,textColor=colors.HexColor(INK),spaceAfter=10))
styles.add(ParagraphStyle(name='SubWM',fontName='Helvetica-Bold',fontSize=12,leading=16,textColor=colors.HexColor(TEAL),spaceBefore=8,spaceAfter=6))
styles.add(ParagraphStyle(name='SmallWM',fontName='Helvetica',fontSize=8,leading=11,textColor=colors.HexColor(MUTED),spaceAfter=10))
styles.add(ParagraphStyle(name='CellWM',fontName='Helvetica',fontSize=8.6,leading=11.7,textColor=colors.HexColor(INK)))
styles.add(ParagraphStyle(name='CellHeadWM',fontName='Helvetica-Bold',fontSize=8,leading=10.5,textColor=colors.white))
def txt(s):return escape(s).replace('\n','<br/>')
def pp(t,style='BodyWM'):return Paragraph(txt(t),styles[style])
PW,PH=595.44,841.68;W=PW-100;story=[]
for idx,p in enumerate(pages):
    if idx:story.append(PageBreak())
    story.extend([pp(p['title'],'TitleWM' if idx==0 else 'HeadWM'),pp(p['subtitle'],'SmallWM')])
    for kind,data in p['blocks']:
        if kind=='p':story.append(pp(data))
        elif kind=='b':story.append(Paragraph('• '+txt(data),styles['BodyWM']))
        elif kind=='h':story.append(pp(data,'SubWM'))
        elif kind=='i':
            path,cap=data; iw,ih=Image.open(path).size
            story.extend([RLImage(path,width=W,height=W*ih/iw),Spacer(1,6),pp(cap,'SmallWM')])
        elif kind=='t':
            heads,rows,widths=data; cells=[[pp(h,'CellHeadWM') for h in heads]]+[[pp(x,'CellWM') for x in r] for r in rows]
            t=Table(cells,colWidths=[W*x for x in widths] if widths else None,repeatRows=1,hAlign='LEFT')
            t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor(INK)),('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.HexColor('#EDF3F6'),colors.white]),('VALIGN',(0,0),(-1,-1),'TOP'),('LEFTPADDING',(0,0),(-1,-1),8),('RIGHTPADDING',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),8),('BOTTOMPADDING',(0,0),(-1,-1),8),('LINEBELOW',(0,0),(-1,0),1,colors.HexColor(TEAL))]));story.extend([t,Spacer(1,10)])
def decor(c,doc):
    c.setStrokeColor(colors.HexColor(TEAL));c.setLineWidth(2);c.line(50,PH-36,PW-50,PH-36)
    c.setFont('Helvetica',8);c.setFillColor(colors.HexColor(MUTED));c.drawString(50,PH-26,'WILD MAGIC ACADEMY / DESIGN DRAFT 1.0');c.drawString(50,26,'14 NOVEMBER 2026  |  CONCEPT + PROTOTYPE PLAN');c.drawRightString(PW-50,26,str(doc.page))
pdf=SimpleDocTemplate(str(ROOT/'Wild_Magic_Academy_Design_Draft.pdf'),pagesize=(PW,PH),leftMargin=50,rightMargin=50,topMargin=55,bottomMargin=48,title='Wild Magic Academy — Game Design Draft',author='Game concept collaboration')
pdf.build(story,onFirstPage=decor,onLaterPages=decor)
print('Created',len(pages),'planned sections')
for f in ROOT.glob('*'): print(f.name,f.stat().st_size)
