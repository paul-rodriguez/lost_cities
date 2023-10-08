#! /usr/bin/env python3

cardValueBits = 10
colorBits = 5
cardBits = colorBits + cardValueBits
handBits = 8 * cardBits
playedCardDestinationBits = 1
drawnCardSourceBits = 1 + colorBits
turnHasBeenPlayedBits = 1
previousTurnBits = cardBits + playedCardDestinationBits + drawnCardSourceBits

maxNumberOfTurns = 120 #This is arbirary "large enough"
historyBits = (turnHasBeenPlayedBits + previousTurnBits) * maxNumberOfTurns

cardFromHandBits = 8
inputBits = historyBits + handBits
outputBits = cardFromHandBits + drawnCardSourceBits

print(f"Input bits: {inputBits}; output bits: {outputBits}")

