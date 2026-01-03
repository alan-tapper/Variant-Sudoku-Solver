import type { NextApiRequest, NextApiResponse } from 'next'

const MOCK_PUZZLES = [
  {
    id: 'b0d10a03-6a08-41e6-957a-45872d4d0a4b',
    name: 'Example Standard Puzzle',
    author: 'seed',
    variant: 'standard',
    difficulty: 3,
    board: [[0,6,0,0,0,0,0,1,0],[0,0,0,6,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[1,5,7,0,0,0,0,0,0],[0,9,6,0,0,0,2,0,8],[2,0,0,1,6,9,0,5,0],[4,1,0,3,0,7,0,6,0],[0,2,0,5,1,0,3,7,0],[7,0,3,4,0,0,1,8,0]]
  }
]

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const {
    query: { id },
  } = req

  const backend = process.env.BACKEND_API_URL
  if (backend) {
    try {
      const r = await fetch(`${backend.replace(/\/$/, '')}/puzzles/${id}`)
      const data = await r.json()
      return res.status(r.status).json(data)
    } catch (e: any) {
      console.error('backend proxy error', e)
      return res.status(502).json({ error: 'Failed to proxy to backend' })
    }
  }

  const p = MOCK_PUZZLES.find((x) => x.id === id)
  if (!p) return res.status(404).end()
  res.status(200).json(p)
}
