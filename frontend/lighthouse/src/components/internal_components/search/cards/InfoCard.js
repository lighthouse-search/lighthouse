export default function InfoCard(props) {
    return (
        <div className="query_profile column outline">
            <div className="images row align_items_unset column_gap_4 scrollX">
            </div>
            <div className="information column row_gap_6">
                <div className="column row_gap_6">
                    <h2>Kendrick Lamar</h2>
                    <p className="description font_size_14">Kendrick Lamar Duckworth is an American rapper. Regarded as one of the most influential hip-hop artists of his generation, and one of the greatest rappers of all time, he was awarded the 2018 Pulitzer Prize for Music, becoming the first musician outside of the classical and jazz genres to receive the honor. -- <Link href="https://example.com">Wikipedia</Link></p>
                </div>
                
                <p className="font_size_14"><b>Born</b>	Kendrick Lamar Duckworth June 17, 1987 (age 37) Compton, California, U.S.</p>
            </div>
        </div>
    )
};