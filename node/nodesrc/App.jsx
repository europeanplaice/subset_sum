import * as React from 'react';
import { Helmet } from 'react-helmet'
import { createRoot } from 'react-dom/client';
import Container from '@mui/material/Container';
import Link from '@mui/material/Link';
import Box from '@mui/material/Box';
import Stack from '@mui/joy/Stack';
import Highlight from "react-highlight.js";
import ReactGA from "react-ga4";

import './style.css';

ReactGA.initialize("G-GL0P9YL3TE");
ReactGA.send("pageview");

const colabpath = 'https://colab.research.google.com/github/europeanplaice/subset_sum/blob/main/python/python_subset_sum.ipynb'

let container = document.createElement('div');
container.id = 'app';
let body = document.querySelector('body');
body.appendChild(container);
const root = createRoot(container);

function HeaderBlock() {
  return (
    <div>
      <header class="page-header" role="banner">
        <h1 class="project-name">Subset Sum(dpss)</h1>
        <h2 class="project-tagline">Solves subset sum problem and return a set of decomposed integers.</h2>
        <Link href="https://github.com/europeanplaice/subset_sum" class="btn">View on GitHub</Link>
      </header>
    </div>
  )
}

function App() {
  return (
    <div>
      <Helmet>
        <title>dpss | Python, Rust and WebApp to solve subset sum problem</title>
        <meta name="description" content="This solves subset sum problem and return a set of decomposed integers." />
        <link rel="canonical" href="https://europeanplaice.github.io/subset_sum/" />
      </Helmet>
      <Container maxWidth="md">

        <HeaderBlock />

        <main id="content" class="main-content" role="main">
          <Stack direction="row">
            <Box style={{width: '40%'}}>
              <Link href='find_subset/index.html' underline="none">
                <Box style={{ padding: '2em', margin: '1em', border: 'solid 4px', borderRadius: '8px'}}>
                  <div>
                    <span>Here is an online subset sum solver.</span>
                  </div>
                </Box>
              </Link>
            </Box>
            <Box style={{width: '40%'}}>
              <Link href={colabpath} underline="none">
                <Box style={{ padding: '2em', margin: '1em', border: 'solid 4px', borderRadius: '8px' }}>
                  <div>
                    <span>Here is an Google Colab Notebook subset sum solver.</span>
                  </div>
                </Box>
              </Link>
            </Box>
          </Stack>
          <div class="block">This library is a Rust implementation of an algorithm that solves subset sum problem. It is
            available for
            both Python and Rust.</div>

          <h2 id="what-is-subset-sum-problem" class="subtitle">What is subset sum problem?</h2>

          <p>Solving subset sum problem means finding a list that sums to a particular value. Assuming there is a list of
            integers (such as [1, 2, 3, 6, -9, 11]), and another integer (such as
            6), subset sum problem is the question to answer the subsets that sum to the specified integer. In this
            case, the answer is [1, 2, 3] and [-9, 1, 3, 11].</p>

          <p>For detail information of subset sum problem, please refer to <Link
            href="https://en.wikipedia.org/wiki/Subset_sum_problem">https://en.wikipedia.org/wiki/Subset_sum_problem</Link>
          </p>

          <h2 id="what-is-dpss" class="subtitle">What is DPSS?</h2>
          <p><Link href="https://github.com/europeanplaice/subset_sum">DPSS</Link> provides a tool to solve this problem without
            any specialized math knowledge. This can be used when you know how to get ALL the different solution that adds up
            to the same total.</p>
          <p>It also offers a method to solve multiple subset sum problem. For example, if you are a travel planner, and you
            have to allocate families to some cars, you can count on this tool to make a plan of allocation.
            Given each member of the family are [2, 3, 2, 4, 5, 3, 2] and the capacities of each car are [4, 5, 8, 4],
            you can allocate the families to the cars as follows:</p>
          <Highlight>pattern 1  => [((4) -> [4] == [4])
                        ((4) -> [2 + 2] == [4])
                        ((5) -> [2 + 3] == [5])
                        ((8) -> [3 + 5] == [8])],
          </Highlight><Highlight>
pattern 2  => [((4) -> [4] == [4])
                        ((5) -> [5] == [5])
                        ((4) -> [2 + 2] == [4])
                        ((8) -> [2 + 3 + 3] == [8])]</Highlight>

          <p>And there are some questions posted on stackoverflow that can be sorted out by this tool.
            The questions are...
            <ul>
              <li style={{ MarginBottom: '1em' }}><Link href="https://stackoverflow.com/questions/3420937/algorithm-to-find-which-number-in-a-list-sum-up-to-a-certain-number">Algorithm to find which number in a list sum up to a certain number</Link></li>
              <li style={{ MarginBottom: '1em' }}><Link href="https://stackoverflow.com/questions/42422921/multiple-subset-sum-calculation">Multiple subset sum calculation</Link></li>
              <li style={{ MarginBottom: '1em' }}><Link href="https://stackoverflow.com/questions/4355955/subset-sum-algorithm">Subset Sum algorithm</Link></li>
            </ul>
          </p>

          <h2 id="how-to-use-dpss" class="subtitle">How to use DPSS?</h2>
          <p>The easiest way to use this tool is the <Link
            href="https://colab.research.google.com/github/europeanplaice/subset_sum/blob/main/python/python_subset_sum.ipynb">Google
            Colab Notebook</Link> that I made. I also explain the other ways in <Link
              href="https://github.com/europeanplaice/subset_sum">https://github.com/europeanplaice/subset_sum</Link> .
          </p>

          <p>Or, there is <Link href="find_subset/">a WebAssenbly implementation</Link>.</p>

          <h2 id="what-are-the-applications-of-subset-sum-problem-and-this-tool" class="subtitle">What are the
            applications of subset sum
            problem and this tool?</h2>
          <p>This tool can be used in bank reconciliation. Here is <span><Link
            href="https://colab.research.google.com/github/europeanplaice/subset_sum/blob/main/python/bank_reconciliation.ipynb">a
            Google Colab Notebook</Link></span>
            that shows the example of the usage of DPSS in bank reconciliation. Inside of
            this example, a function of solving multiple subset sum problem is used.
          </p>
          <footer class="site-footer">
            <span class="site-footer-owner"><Link href="https://github.com/europeanplaice/subset_sum">subset_sum</Link> is
              maintained by <Link href="https://github.com/europeanplaice">europeanplaice</Link>.</span>
            <span class="site-footer-credits">This page was generated by <Link href="https://pages.github.com">GitHub
              Pages</Link>.</span>
          </footer>
        </main>
      </Container>
    </div>
  )
}

root.render(<App />)
