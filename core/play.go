package core

func (c *Core) AnythingToPlay() bool {
	if c.toPlay != "" {
		return true
	} else {
		return false
	}
}

func (c *Core) ToPlay(thing string) {
	c.toPlay = thing
}

func (c *Core) Play() string {
	toPlay := c.toPlay
	c.toPlay = ""

	return toPlay
}
