import "../src/components/global.css";
import { Roboto } from 'next/font/google';
import Head from 'next/head';

const roboto = Roboto({
  subsets: ['latin'],
  weight: ['400', '700']
})

export const metadata = {
}

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <link rel="search" type="application/opensearchdescription+xml" href="/opensearch.xml" title="Lighthouse"/>
      <body className={roboto.className}>{children}</body>
    </html>
  )
}
