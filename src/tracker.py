import sys
import math
import subprocess
import pygame
from time import sleep
from pygame.locals import *
 
pygame.init()
 
fps = 60
fpsClock = pygame.time.Clock()
 
width, height = 640, 480
screen = pygame.display.set_mode((width, height))

pattern=[[]]
volumes=[[]]
type=[[]]
song=[[]]

scr=0
patt=0

updown=48
adsr=1

for i in range(16):
    pattern[0].append(0)
    
# Game loop.
while True:
    screen.fill((160, 160, 166))
    if scr==0:
        for i in range(32):
            bow = (i+updown)%12 == 1 or (i+updown)%12 == 3 or (i+updown)%12 == 6 or (i+updown)%12 == 8 or (i+updown)%12 == 10
            if not bow:
                pygame.draw.rect(screen,(200,200,200),(0, 465-i*15, 640, 15))
            
            pygame.draw.line(screen,(80,80,80),(0,465-15*i),(640,465-15*i),1)
        for i in range(36):
            if i%4==0:
                pygame.draw.line(screen,(50,50,50),(100+30*i,0),(100+30*i,480),1)
            else:
                pygame.draw.line(screen,(100,100,100),(100+30*i,0),(100+30*i,480),1)
            

        for i in range(32):
            bow = (i+updown)%12 == 1 or (i+updown)%12 == 3 or (i+updown)%12 == 6 or (i+updown)%12 == 8 or (i+updown)%12 == 10
            if bow:
                pygame.draw.rect(screen, (255,255,255),(0,465-15*i,100,30))
                pygame.draw.rect(screen, (0,0,0),(0,465-15*i,100,30),1)
                pygame.draw.rect(screen, (0,0,0),(0,465-15*i,80,15))
            else:
                pygame.draw.rect(screen, (255,255,255),(0,465-15*i,100,15))
                pygame.draw.rect(screen, (0,0,0),(0,465-15*i,100,15),1)
        for i in range(16):
            if pattern[patt][i] != 0:
                pygame.draw.rect(screen,(0,0,100),(100+30*i,465-15*(pattern[0][i]-updown)+15+11*15, 30,15))


    pygame.draw.rect(screen, (50,100,255),(0,0,640,50))

    pygame.draw.polygon(screen,(0,0,100),((75,15),(75,30),(30+75,30)))
    pygame.draw.rect(screen,(0,0,100),(45,15,30,16))
    pygame.draw.polygon(screen,(0,0,100),((45,15),(15,30),(45,30)))
    if adsr==2:
        pygame.draw.polygon(screen,(0,0,200),((75,15),(75,30),(30+75,30)))
    elif adsr==1:
        pygame.draw.rect(screen,(0,0,200),(45,15,30,16))
    elif adsr==0:
        pygame.draw.polygon(screen,(0,0,200),((45,15),(15,30),(45,30)))

    
    for event in pygame.event.get():
        if event.type == QUIT:
            pygame.quit()
            sys.exit()
        if event.type == KEYDOWN:
            if event.key == K_p:
                for i in range(len(pattern[0])):
                    if pattern[0][i]!=0:
                        res = subprocess.Popen(["./girlsynth",  str(pattern[0][i]) ] )
                    sleep(0.1245)
            if event.key == K_e:
                petrn = []
                for i in range(len(pattern[0])):
                    petrn.append(pattern[0][i] + 0.0 )
                print(petrn)
            if event.key == K_UP:
                updown += 1
            if event.key == K_DOWN:
                if updown > 0:
                    updown -= 1
        if event.type == MOUSEBUTTONDOWN:
            if event.button == BUTTON_RIGHT:
                mp=pygame.mouse.get_pos()
                if mp[0]>100 and mp[0]:
                    if math.floor((mp[0]-100)/30) < 16:
                        pattern[patt][math.floor((mp[0]-100)/30)] = 0
            if event.button == BUTTON_LEFT:
                mp=pygame.mouse.get_pos()
                if mp[0]>100 and mp[0]:
                    if math.floor((mp[0]-100)/30) < 16:
                        pattern[patt][math.floor((mp[0]-100)/30)] = math.floor(32-mp[1]/15)+1+updown+11
                        res = subprocess.Popen(["./girlsynth",  str(math.floor(32-mp[1]/15)+1+updown+11) ] )
                for i in range(3):
                    if mp[0] > 15 + 30*i and mp[0] < 15 + 30*(i+1) and mp[1] > 15 and mp[1] < 30:
                        adsr=i



        
    # Update.
    
    # Draw.
    
    pygame.display.flip()
    fpsClock.tick(fps)



"""
let TITLE = Tune
    {
        bpm:150.0,
        tracks:1,
        instruments: vec![
            INST
        ],
        patterns: 
        vec![
            PATTERNS
        ],
        volumes:
        vec![
            VOLUMES
        ],
        types:
        vec![
            TYPES
        ],
        song:
        vec![
            SONG
        ]
    };
"""