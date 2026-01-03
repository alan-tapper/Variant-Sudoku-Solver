import React from 'react'

export default function PuzzleGrid({ board }: { board: number[][] }) {
  return (
    <div style={{ display: 'inline-block', border: '1px solid #333', padding: 8 }}>
      {board.map((row, i) => (
        <div key={i} style={{ display: 'flex' }}>
          {row.map((cell, j) => (
            <div key={j} style={{ width: 28, height: 28, display: 'flex', alignItems: 'center', justifyContent: 'center', border: '1px solid #ddd', margin: 1 }}>
              {cell === 0 ? '' : cell}
            </div>
          ))}
        </div>
      ))}
    </div>
  )
}
