# Project Goals
- Determine the optimal strategy to play Knock
- Determine an optimal incentive/decentive for knocking

# Knock (Card Game) Rules
- Setup:
	- Uses standard 52 card deck
	- Each player is dealt 4 cards face-down in a 2x2 grid.
	- Remaining cards placed placed face-down in stock pile.
	- Each player may look once at the two closest cards at the beginning of the round.
- Gameplay: Starting from the player to the dealer's left, the players take turns, each of which consist of either:
	- Draw and discard 
		1. Draw from the either the stockpile or the discard pile
			- This card may only be viewed by the player who drew it
		2. Swap the drawn card with a card in the grid, placing the new one face down.
			- This action is only optional if the card came from the stock pile. If the card is from the discard pile, the player is required to swap out the card.
		3. Discard the card in the player's hand face-up
	- Knock
		- The player forfeits their turn
		- Other players continue their turns for one last "lap"
		  - They may not knock
		- Once all other players have played, all the cards are revealed
			- This is the end of the game
- Goal: Maximize the total number of points in your grid.
- Scoring:
	- 1 through 10 are their face value
	- The face cards (Jack, Queen, King) are all 10
- Variations:
	- A different size of grid may be used
	- A different number of initially revealed cards may be used
	- Knocking may have penalties and/or bonus
	- This is actually just a variation of [Golf](https://en.wikipedia.org/wiki/Golf_(card_game))