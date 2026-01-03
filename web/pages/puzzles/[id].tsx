import { useRouter } from 'next/router'
import useSWR from 'swr'
import PuzzleGrid from '../../components/PuzzleGrid'

const fetcher = (url: string) => fetch(url).then((r) => r.json())

export default function PuzzlePage() {
  const router = useRouter()
  const { id } = router.query
  const { data: puzzle, error } = useSWR(id ? `/api/puzzles/${id}` : null, fetcher)

  if (error) return <div>Failed to load</div>
  if (!puzzle) return <div>Loading...</div>

  return (
    <div style={{ padding: 20 }}>
      <h1>{puzzle.name}</h1>
      <p>Variant: {puzzle.variant} — Difficulty: {puzzle.difficulty}</p>
      <PuzzleGrid board={puzzle.board} />
    </div>
  )
}
