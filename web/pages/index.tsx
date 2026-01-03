import Link from 'next/link'
import useSWR from 'swr'

const fetcher = (url: string) => fetch(url).then((r) => r.json())

export default function Home() {
  const { data: puzzles, error } = useSWR('/api/puzzles', fetcher)

  if (error) return <div>Failed to load</div>
  if (!puzzles) return <div>Loading...</div>

  return (
    <div style={{ padding: 20 }}>
      <h1>Variant Sudoku — Puzzles</h1>
      <ul>
        {puzzles.map((p: any) => (
          <li key={p.id}>
            <Link href={`/puzzles/${p.id}`}>{p.name} — {p.variant} — difficulty {p.difficulty}</Link>
          </li>
        ))}
      </ul>
    </div>
  )
}
