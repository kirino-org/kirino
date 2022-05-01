package core

import "time"

type Task struct {
	ID   int    `json:"id"`
	Name string `json:"name"`

	LastRun  time.Time     `json:"last_run"`
	RunEvery time.Duration `json:"run_every"`

	Func func(c *Core) `json:"-"`
}

func (c *Core) Task(id int) *Task {
	return c.task[id]
}

func (c *Core) Tasks() []*Task {
	return c.tasks
}

func (c *Core) AddTask(t *Task) {
	c.tasks = append(c.tasks, t)
	c.task[t.ID] = t
}
