import sys,json,shutil
from pathlib import Path
sys.path.insert(0,'/tmp/wild-magic-doc-tools')
import pymupdf
from PIL import Image
from reportlab.pdfgen import canvas
from reportlab.lib.utils import ImageReader
from reportlab.lib.colors import HexColor
from reportlab.platypus import Paragraph
from reportlab.lib.styles import ParagraphStyle
from xml.sax.saxutils import escape
ROOT=Path('/home/abe/minidupe/documents');BUILD=ROOT/'.build';ART=BUILD/'expanded-art';ART.mkdir(exist_ok=True)
manifest=json.loads((BUILD/'expanded-art-manifest.json').read_text())
for e in manifest:
 shutil.copy2(e['path'],ART/(e['key']+'.png'))
(ART/'prompts.json').write_text(json.dumps(manifest,indent=2))
INK='#202641';TEAL='#087F8C';MUTED='#65718A';W,H=841.89,595.28
style=ParagraphStyle('note',fontName='Helvetica',fontSize=8.8,leading=11.6,textColor=HexColor(INK))
small=ParagraphStyle('small',fontName='Helvetica',fontSize=8,leading=10,textColor=HexColor(MUTED))
def para(c,text,x,y,w,st=style):
 p=Paragraph(text,st);ww,hh=p.wrap(w,200);p.drawOn(c,x,y-hh);return hh
plates=[
('base-animation','16 / Base spells in motion','Four rows, read top to bottom; each row progresses from left to right.',[
('FIREBALL','Anticipate > release > surface hit > brief explosion. The quick-cast projectile ends on its first enemy or surface.'),
('GUST','Brace > intercept > turn > return. Quick Gust sends the projectile toward its original caster; aimed redirection belongs to charged Gust.'),
('STONE FIST','Wind-up > short punch > hit > debris clears. The rocky fist has limited reach; it is not a roaming projectile.'),
('FROST BOLT','Aim > launch > slowing hit > frost fades. Slowing is allowed; hard freezing is not part of the chosen design.')]),
('fusion-fire','17 / Fusion studies: fire combinations','Three rows: Flame Tornado, Magma Bomb, Steam Burst. Read each sequence left to right.',[
('FIRE + GUST / FLAME TORNADO','Gather > form > lift > escape. A moving vortex damages and displaces enemies; they can still cast, move and dash out.'),
('FIRE + STONE / MAGMA BOMB','Gather > arc > impact > burning ground. A heavy projectile creates a brief blast and a temporary floor hazard.'),
('FIRE + FROST / STEAM BURST','Gather > burst > obscure > clear. Area damage and knockback produce localized steam that affects everyone’s view.')]),
('fusion-control','18 / Fusion studies: control & mobility','Three rows: Orbiting Stones, Ice Jet, Glacier Wall. Read each sequence left to right.',[
('GUST + STONE / ORBITING STONES','Gather > orbit / intercept > aim > launch. Defense is limited. Repeating the fusion input launches early; remaining stones auto-launch at expiry.'),
('GUST + FROST / ICE JET','Gather > dash > pass through enemy > stop at terrain. Each enemy takes at most one hit per dash; the slippery trail affects all players.'),
('STONE + FROST / GLACIER WALL','Gather > preview > place > block / break. Place within limited range. Aim stays adjustable during wind-up and locks when the wall appears.')]),
('character-design','19 / Wizard design & animation language','A shared base silhouette, distinct player colors and expressive physical acting.',[
('TOP ROW / COSMETIC VARIANTS','Red, teal, gold and violet accents identify players. Hats and accessories personalize the shared wizard base; their designs are exploratory.'),
('LOWER STUDIES / ACTION POSES','Idle, run, jump, dash, cast, recoil, defeat and victory explore readable body language. This is not a production-ready animation or turnaround sheet.'),
('GAMEPLAY GUARDRAILS','Equal body dimensions and gameplay capabilities. No persistent elemental glow that reveals an uncast spell. Future magical species are outside this initial character set.')]),
('courtyard-art','20 / Courtyard artwork & material studies','Storybook atmosphere, pixel gameplay elements and a clear foreground combat plane.',[
('MAIN ARTWORK / MOOD','Bright academy rivalry, three height levels and spectacular elemental bursts. Keep full character silhouettes readable; this is mood art, not a replacement for the arena schematic.'),
('DETAIL STUDIES / MATERIALS','Explore a cracking wooden bridge, its glowing rebuild warning and a sheltered mana alcove. The warning must precede the bridge becoming solid.'),
('SPECTACLE / PERFORMANCE','Favor short bright bursts over dense lingering particles. Scale decorative effects on weaker phones while preserving attacks, hazards and tactical steam visibility.')])]
append=BUILD/'visual-atlas.pdf';c=canvas.Canvas(str(append),pagesize=(W,H));c.setTitle('Wild Magic Academy — Expanded Visual Atlas')
for idx,(key,title,subtitle,notes) in enumerate(plates):
 c.setFillColor(HexColor(MUTED));c.setFont('Helvetica',8);c.drawString(36,H-24,'WILD MAGIC ACADEMY / EXPANDED VISUAL EDITION 1.1')
 c.setStrokeColor(HexColor(TEAL));c.setLineWidth(2);c.line(36,H-33,W-36,H-33)
 c.setFillColor(HexColor(INK));c.setFont('Helvetica-Bold',20);c.drawString(36,H-61,title)
 c.setFillColor(HexColor(MUTED));c.setFont('Helvetica',9);c.drawString(36,H-79,subtitle)
 path=ART/(key+'.png');iw,ih=Image.open(path).size
 # Keep the entire generated sheet visible; do not crop or modify artwork.
 maxw=W-72;maxh=H-210;s=min(maxw/iw,maxh/ih);dw,dh=iw*s,ih*s
 c.drawImage(str(path),(W-dw)/2,113+(maxh-dh)/2,width=dw,height=dh,mask='auto')
 gap=16;cw=(W-72-gap*(len(notes)-1))/len(notes)
 for j,(head,body) in enumerate(notes):
  x=36+j*(cw+gap);para(c,'<b>'+escape(head)+'</b><br/>'+escape(body),x,101,cw)
 c.setFillColor(HexColor(MUTED));c.setFont('Helvetica',7.5);c.drawString(36,17,'Generated concept studies, not final assets or frame timings. Mechanics follow the design text; future features are not prototype scope.')
 c.drawRightString(W-36,17,str(17+idx));c.showPage()
c.save()
pdf=ROOT/'Wild_Magic_Academy_Design_Draft.pdf';backup=BUILD/'Wild_Magic_Academy_Design_Draft_original.pdf'
if not backup.exists():shutil.copy2(pdf,backup)
doc=pymupdf.open(backup)
# Preserve the base design, while making the expanded PDF's provenance accurate.
p=doc[15]
replacements=[
('Editable document and PDF share this source content','Base design reference; expanded visual atlas follows on pages 17-21.',8,0),
('Wild_Magic_Academy_Design_Draft.docx is the editable master.','The Word document retains the original 16-section design draft. This PDF expands that draft with five new concept-art sheets on pages 17-21: spell animations, six fusions, character studies and courtyard artwork. Supporting artwork and generation prompts are retained in .build/expanded-art.',9.7,12)]
for needle,new,size,pad in replacements:
 blocks=p.get_text('blocks');b=next(b for b in blocks if needle in b[4]);r=pymupdf.Rect(b[:4]);p.add_redact_annot(r,fill=(1,1,1));p.apply_redactions(images=0)
 r.x1=540;r.y1+=pad+12
 ret=p.insert_textbox(r,new,fontname='helv',fontsize=size,color=(.13,.15,.25),lineheight=1.35)
 if ret<0:raise RuntimeError('Replacement text overflow')
# Small cover note in the free space beneath the original introduction.
p=doc[0];p.insert_text((56,704),'Expanded visual edition 1.1: five new illustrated sheets on pages 17-21.',fontsize=9,color=(.03,.50,.55))
atlas=pymupdf.open(append);doc.insert_pdf(atlas)
meta=doc.metadata;meta.update({'title':'Wild Magic Academy — Expanded Illustrated Design Draft 1.1','subject':'Game concept, prototype plan, spell animation and fusion concept art','keywords':'magic, arena, Android, local Wi-Fi, concept art, animation, fusions'});doc.set_metadata(meta)
out=BUILD/'expanded-final.pdf';doc.save(out,garbage=4,deflate=True);doc.close();shutil.copy2(out,pdf)
# Validate content and preserve review thumbnails.
doc=pymupdf.open(pdf);assert len(doc)==21
for i in range(16,21):
 p=doc[i];assert len(p.get_images())>=1;assert len(p.get_text())>650
 for b in p.get_text('blocks'):
  assert b[0]>=0 and b[1]>=0 and b[2]<=p.rect.width+1 and b[3]<=p.rect.height+1,(i,b)
 p.get_pixmap(matrix=pymupdf.Matrix(1,1),alpha=False).save(BUILD/('visual-review-'+str(i+1)+'.png'))
print('Updated PDF:',pdf);print('Pages:',len(doc));print('New generated sheets:',len(plates));print('Bytes:',pdf.stat().st_size)
