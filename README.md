# algo-lab
Laboratory for algorithms. Use experimental approach to study algorithms. Implementations in rust.

# Main goal for the experiments
The main goal for this project is to get insights and learn more about the 
algorithms and techniques being studied. Thus, I think a good general script 
to follow for each experiment is: (
1. Implement and make sure the algorithm is correct.
2. Visualize it. It is interesting if we have visual feedback from the algorithm working.
3. Test performance using criterion. Try to compare with different implementations, including the following 
programming languages: rust/C/C++.
For the step two it is interesting to have some kind of macro that extracts information from the
execution process and returns this information as textual data. This data will be used to generate
visual elements about the data structure being studied.

# Draft (General ideas that will be used to make the experiments possible)
- the tests will be carried on algorithms or programs that are correct, thus, they
must be thoroughly tested before the experiments.
- The experimental approach to the study of algorithms is in between the theoretical
and empirical approach. It differs from empirical approach because it treats
algorithms as laboratory subjects, including control of parameters, isolation
of key components, model building, and statistical analysis.
- The algorithm design hierarchy: system structure, algorithm and data structure
design, implementation and algorithm tuning, code tuning, system software, and
platform and hardware. Each of those dimensions represent broad strategies for
speeding up algorithms.
- Among the fundamental tools that will be implemented with the algo lab, I want to
  implement a tool for generating random elements in a close way to the serde crate.
  It will take advantage from derive macros to derive random properties for enums,
  structs, vec, std types, etc. It will allow the use of attributes to fine tune
  the behavior of the random generation.
- An important aspect is the seed of random generators. They must be easily defined by
  the tester.
- The framework must make the creation of the following experiment very easy:
  trials 3
  nmin 50
  nmax 90
  nstep 20
  seed 77271179
  performance indicators ...
  file formats
  source ...
  destination ...

  A default value is defined for missing fields.
- Output generated should be in very easy to use format. Performance data should
  be in matrix format: one line per trial, with each line containing both the
  experimental parameters (input size, trial number, code version, etc) and the
  performance measurements for that trial. The matrix must contain a header (names
  of each column) and a type header (types for each column) rows.
- For algorithmic engineering projects, how to automate the process of documenting
  the factorial combinations?
## The experiment process
- The experiment is a loosely cyclical process of planning and execution. The 
steps are not necessary in order, but in the end, after refinement, they must 
create good artifacts, being either good engineered algorithms or insights and
knowledge for learning purpose or for academical purpose.
- The steps are: 
    1. Plan the experiment:
        a. Formulate a question
        b. Build the test environment (test program, input instances and instance
        generators, measurement tools and packages, and data analysis software)
        c. Design the experiment to address the question in hand (for example, 
        what properties are to be measured, what input categories are applied,
        which input sizes are measured, how many random trials are run, etc)
    2. Execute the experiment:
        a. Run the tests and collect the data
        b. Apply data analysis to obtain information and insights (if the question
        was not answered, go back to the planning stage)
        c. Write and publish the results.
- From a subject, multiple question may arise. Each question may originate one 
experiment. Thus, multiple experiments may benefit from a common set of sources 
(test program, input instances, etc)
## Experimental Goals
The main goals of the experiments that will be made in this lab are:
1. Answer the question proposed for each experiment
2. Be reproducible
    a. Must be correct (data generated accurately reflect the property being 
    studied)
    b. Must be valid (conclusions from correct interpretation)
3. Efficient (general results)
4. (Optional) Newsworthyness, which means that it generates value to academia
research. This will not be a main goal because much of the experiments carried
here will be for learning purpose (for example, reproducing results from well
known experiments)

## Pilot experiment vs Workhorse experiment
- The pilot experiment is an exploratory experiment and may be less formal and
careful than the workhorse. 
- The pilot experiment helps answering questions like:
what assumptions are valid and what are not, what are the most important relationships 
and properties, what to expect from the test environment (how long to execute, 
number of samples, largest input feasibly measured, etc). 
- The workhorse experiment is much more precise and specific in terms of description
of the preconditions (for example, comparing the performance for data structures
A and B with the same implementation etc)

## Be aware of supurious result
- Spurious result happens when the experimenter mistakenly attributes some outcome
to the wrong cause. Some examples are:
    1. Ceiling and floor effects: happens when the experiment is too easy or
    too hard to make it possible any difference between the algorithms being tested
    to arise. For example, if the input is too small, all the algorithms may perform
    very close or if the input is too big, all of them may perform very badly.
    2. Experimental artifacts
        a. Time measurement
        b. Bugs (a very thorough suit of tests for the candidates being tested is
        necessary to avoid incorrect implementations)
        c. Pseudorandom number generations that inject bias to the result
        d. Floating point precision errors


## Basics
- Performance metric: a dimension of algorithm performance that can be measured
(i.e. time, solution quality [like the number of combinations from a set of 
elements that have some specific property], space usage)
- Performance indicator: a quantity associated to a performance metric that can
be measured in an experiment. (metric: time -> indicator: CPU time)
- Parameter: any property that affects the value of a performance indicator.
    - Categorical parameters:
        - The algorithm or the test program implemented
        - Input instances (input size, etc)
        - Environment parameters (compiler, operating system, etc)
- Factor: a parameter that is explicitly manipulated in the experiment.
- Level: A value assigned to a factor in an experiment. (for example, one 
experiment could have the following factors: A, B, C, and for each factor, a 
group of values: A -> [1, 2, 3], B -> [b1, b2], etc)
- Design point: a particular combination of levels to be tested. Using the 
previous experiment, we could have the following named tuple: (A: 1, B: b2, etc)
- Trial or test: One run of the test program at a specific design point, which
produces a measurement of the performance indicator.
- Fixed parameter: a parameter held constant through all trials
- Noise parameter: a parameter with levels that change from trial to trial in an
uncontrolled or semi-controlled way.

### Selecting Inputs
Input instances may be collected from real-world data or generated by programs.
There are some aspects that guide the choice of input data:
- Stress-test: the goal is to find bugs and reveal artifacts. Useful mainly for
algorithms that may generate non optimal results. Otherwise, those input cases
will be used for testing the correctness of the implementation. 
- Worst-case and bad-case instances: assess algorithm performance boundaries.
- Random inputs: typically controlled by a small number of parameters, using
random numbers to fill details. Useful, for example, to measure average case of
algorithms.
- Structured random inputs:
    - Algorithm-centered generators: built with parameters that exercise 
    algorithm mechanisms.
    - Reality-centered generators: capture properties of real-world inputs.
- Real instances: collected from real world.
- Hybrid instances: combination of real-world instances with generated 
components.
- Public testbed

    Good test beds should have at least one of the following characteristics:
    - features that are relevant to algorithm performance
    - provable properties
    - permit controlled experiments using parametrization
    - typical of real-world application domains
    - display algorithm performance on a good variety of both applied and 
    theoretical scenarios
    - yield insights into underlying algorithm mechanisms

### Main categories for motivating questions
1. Assessment: experiments that look at general properties, relationships, and
ranges of outcomes. Can be used to explore better questions. Ex. What input 
properties affect performance the most?
    - Try double experiments for a quick assess of function growth. Double 
    experiments are basically experiments in which the input size sequence grows
    exponentially, doubling for each element. If the cost does not change,
    it is constant in relation to input size, if the cost increments
    by constant, it grows like O(log(N)). If the cost doubles as N doubles, it
    is linear. If C(N)/N grows by a constant, then, C(N) is O(N.log(N)). If
    the cost quadruples each time N doubles, C is quadratic, etc.
    - Determine the convergence of algorithms in the context of 
    iterative-improvement heuristics and stochastic algorithms. A good stopping
    rule must be chosen.
    - Full factorial designs maximize the information gained from one experiment (for example,
      testing an insertion sort implementation in regard to loop motion and sentinel
      use allow us to see that: -- is the worst, +- improves, -+ makes it worse,
      ++ improves the most. If we did not use the factorial approach, we may think
      that sentinel optimization was bad).
2. The horse race: Looks for winners and losers in the space of implementation
ideas. Ex. What is the best implementation for some algorithm?
3. Fitting functions: we know the form of some cost function and we want to 
determine its constant values.
4. Modeling: we want to find the correct function family to describe a given 
cost.


## Experimental design template

Below is a template for an experiment:
- Question: (Random) How does average color count in random graphs depend on number of
  iterations I?
- Performance indicators: color count
- Factors: Random graphs(n, p), algorithm parameter I.
- Levels: n=200..800 (+100), p=0.25..1 (+0.25), I=n^2
- Trials: 25 per design point
- Design points: full factorial (4x7=28)
- Outputs: All factors, color count every 100 iterations, number of nodes per color
  at beginning and end of each trial, full coloring at end of each trial.

## Performance indicator

### Time
Time performance indicators can be aimed at different points on the scale between abstract algorithms (e.g. using instruction count)
and instantiated code (e.g. using CPU time to fine tune the implementation).
For academic research, it is important to always include some platform independent measurements in
the paper to allow replicability.

It is important to notice that when we are dealing with memory hierarchy effects on the program 
being tested, unexpected behavior may arise. If the memory footpring of the program exceeds some
boundary, it may generate memory related discontinuity. Also, not all memory waits are considered
as CPU time.

Related to cache, we can have a cold start (empty the caches or load it with irrelevant data) or a warm
start (load useful data to the cache).

When dealing with concurrency and possible parallelism with multiple cores, elapsed time may be prefered.
One strategy is to measure an initial timestamp from a single threaded initial process, run the process 
concurrently, creating new threads as needed, after all threads have finished, measure the end timestamp
from the original parent process.

#### Instruction Count
One simple way to measure performance is to measure the number of time some specific instruction was
executed. This can be done by using integers to store the number of times a line was executed or a function
was called. Rust macros could aid this process.
#### Time Measurement
There are two approaches to measure process time: 
1. Elapsed time (real time, wall clock time): compare timestamps from start to end. This
   measure technique is very sensitive to system load.
2. CPU time (interval time): measured by the OS for each process. This measurement technique
   becomes more precise with longer process time. 

Guidelines for measuring time:
- For single process with a minimum duration (> 1s), prefer CPU time. For shorter duration,
  prefer high precision elapsed time.
- Use lightly loaded systems: kill all competing applications, background processes, avoid generating
  keystroke interrupts, and screen update events.

A good tool to use for this purpose is the unix command `time`.

#### Code Profilers
When a computational experiment is developed to study a section of code rather than the process,
we can use (1) - code counters, (2) - timein/timeout instructions, or (3) - code profilers (e.g. gprof,
valgrind).

An important observation about compiler-based code profilers (e.g. gprof, valgrind) is that they should not
be used in combination with compiler optimization.

#### Tools for Measuring Time
- Unix:
  - CPU time: sysconf(), clock_getres(), profil(), getrusage()
  - Elapsed time: gettimeofday()
  - CPU usage: top, ps
  - Memory: top, free, vmstat. valgrind
  - IO: iostat
- Windows
  - Elapsed time: GetTickCount, GetTickCount64, timeGetTime, timeGetSystemTime
  - CPU usage: Usage

### Solution Quality
It depends on the problem and there is not too much we can do in terms of the framework.
Use section 3.2 Solution Quality (page 94) as reference.

## Tuning Algorithm and Code
There are two main strategies for tunning code/algorithm:
1. Reduce instruction count
2. Reduce instruction time: identify time-expensive instructions and reduce their counts or their time. Most
   code/algorithm tuning focus on the instruction count deduction. Instruction time can be considerably reduced
   because of temporal locality and spatial locality principles.

After making the experimentation, if you decide to tune the algorithm/code, there are some strategies below: 
- Algorithm tuning:
  - Branch-and-bound: interesting for exhaustive-search algorithms. The idea is to insert a test to compare
    the minimum (or maximum) cost found so far to a lower-bound (upper-bound) estimate on the final cost of
    a partially constructed solution. If the lower-bound (upper-bound) is greater than minimum (max) so far,
    further recursion is abandoned.
  - Propagation: replaces a full computation in each recursive stage with an incremental computation that
    passes partial results as parameter.
  - Preprocessing: add work before the algorithm begins (e.g. a reasonable good initial guess), to save
    work when the algorithm executes.
  - Memoization: store results to avoid recaltulating them in the future.
  - Finesse a calculation: replace an expensive exact calculation with an inexpensive bound or approximation,
    in such a way that the overall result is unchanged.
  - Filtering: avoid inserting an element into a data structure if the element cannot affect the outcome
    of the computation.
  - Recursive algorithm tuning:
    - Pruning: implement tests to skip recursive calls when possible. Boost the strength of these tests
      by using preprocessing or by changing computation order.
    - Control subproblem size: remove elements from subproblems before recurring; add or subtract work to
      balance subproblems.
    - Shrink cost per stage: make a recursive stage faster (e.g. instead of redoing computation, pass
      partial results as parameters).
    - Hybridize a recursive program: use alternative approaches depending on some conditions of the
      recursive call (e.g. use insertion sort for small recursive calls in quicksort).
  - Iterative algorithm tuning:
    - Skip expensive operations: add tests to avoid expensive operations in the main loop.
    - Loop abort: add a test to stop a loop earlier.
  - Data structure tuning: (1) - find right balance of costs among all operations, according to the
    frequency in which they are invoked; (2) - take advantage of locality
    - Customizing the data structure: select a data structure implementation that best match its pattern
      of use.
    - Change the input presentation: instead of changing the code to match the input, change the input
      presentation to match the code.
    - Self-tuning data structures: when inputs are not known in advance, consider self-tunning data
      structures that respond to input properties observable at runtime.
- Code tuning: it is a lower level tuning than algorithm tuning. It looks at loop and procedures
  instead of algorithm paradigms, and at memory layouts instead of data structures. Instead of focusing
  on reducing the number of times a code block is executed, code tuning focuses on making a code block
  faster by rewriting source code, making the compiler emits fewer machine instructions in the block.
  Another important aspect about code tuning is that code tuning intuition fails in modern environment
  (with hardware complex behavior and compiler optimizations), thus every change must be tested. Generally,
  compilers are better than humans on code tuning.
  - Loop tuning:
    - Remove code from loop: most compilers will do this for obvious situations. But for non-trivial cases,
      it can be benefic to remove unnecessary code from inner loop (e.g. remove target element assignment
      from insertion sort's inner loop).
    - Sentinel: add an extreme element in some extreme of the array being used by a loop to avoid unnecessary
      computation.
    - Loop fusion
    - Loop fission
    - Loop unroling: execute blocks of subsequent iterations to avoid loop header overhead.
    - Unswitching: move a decision from inside a loop
    - Nesting busy loops: busy loops should be inside
  - Procedure tuning:
    - Inlining functions
    - Collapse procedure hierarchies: loop unrolling for recursive call
    - Paremeters and local variables: pass by reference huge parameters and shrink the memory footprint
      of local variables to avoid register spill.
  - Objects and memory structures:
    - Avoid object resizing: allocate a large enough object from the start to avoid unnecessary resizes
      in the future.
    - Mutability: when performance is needed, mutability can be very helpful because it avoids creation of
      a new object when the original is changed. Other strategy related to mutability is recycling objects.
    - Static bindings: moving computation form runtime to compile time can be benefical for performance. But, when
      the object is frequently accessed it can be better to have it in a dynamic context for better memory locality. 
- Tuning for memory:
  - Shring the data footprint: avoid memory leaks, remove unneeded data, use compression, etc. This approach is the
    opposite to memoization. Memoization exchanges memory by less instruction count.
  - Cache efficience: organize the program so that access to memory elements displays as much spatial and temporal locality
    as possible.
    - Reuse as soon as possible
    - Respect memory layouts: for example, loop through matrices by row/column if it is row/column-major
- Tuning for IO:
  - Minimize open/close operations: e.g. store data in a big file instead of multiple smaller files.
  - Reduce latency: latency is the wait for the disk and transfer time is the wait for the data to transfer. Reduce
    latency by using few data transfer operations.
  - Decouple I/O and instruction execution: IO buffering and threading can help with that
  - Exploit locality: organize data in disk to make best use of spatial and temporal locality.
- Tuning by concurrency: below are some candidates for good tuning by concurrency
  - Divide and conquer algorithms
  - branch-and-bound
  - array based computations
  - Decoupling slow processes
  - Minimize threading overhead: it is better to have few long running threads than many short lived threads.

### Framework to tune algorithms
The book describes a framework to tune algorithms. Check sec. 4.3 - The tuning process (page 146) for more details

## The Toolbox

### Interesting references
- [Stony Brook Algorithm Repository](https://algorist.com/algorist.html)
- [LEDA](https://algorithmic-solutions.info/site/leda/index.htm)
- [DIMACS](http://dimacs.rutgers.edu/index.php/)
- CGAL (Computational Geometry Algorithms Library)
- The Stanford GraphBase
- ACM Journal of Experimental Algorithms
- The R Project for Statistical Computing

### Test program
- Test program: the program targe of the tests. It may or may not be identical to the application program.
  Generally, it will be instrumented with additional code to help in the testing process.
- Test apparatus (harness or scaffolding): extra wrapper code added to an implementation to support experiments.

### Random generator
A good RNG must follow the two properties: uniformity and independence. Linear congruential generators (those
used in programming languages) fail in the latter. For that reason they are called pseudorandom number 
generators.
Some hints for using RNG:
- Use huge seeds or run the RNG multiple times before the experiment if the seed is not controlled.
- Low order bits tend to be less random.
- Avoid using sequential RNG for generating compound data. For example, (x, y, z) generated using
  sequential random numbers could generate a pattern of serial correlation. Use different streams
  of random numbers with different seeds.
- All the complex combinations of random elements generated for algorithmic experiments are
  generated from RNGs.
- Typical random outputs: float [0, 1), int[0, m), permutation, sample (subset of k elements
  drawn from n elements, with or without), ordered integer/real samples, resevoir samples,
  random combinatorial objects (created from random sample).
- Nonuniform random variables are also important for experimenting purposes. They are described
  by probability distributions.
- Random structures and objects: there are multiple techniques for generating different types of
  objects like: random uniform graph, nearest-neighbor graph, random proximity graph, etc, trees, etc.
- Nonindependence: some generated inputs must depend on previous results and thus need to be generated
  following this restriction. Some techniques can be used to achieve that.

## Creating Analysis-Friendly Data
- Magnify response and minimize variance.
- Some hints for better experiments: design the experiments to maximize the information content in
  the data: aim for clear views of simple relationships.
  - Do more trials
  - Expand the range of `n` values being tested
  - Don't summarize prematurely
  - Don't use lossy performance indicator
  - Prefer narrow performance indicator: prefer count of a single operation instead of CPU time, for exemple.
- In some situations, it is possible to reduce the variance of an experiment without increasing more trials
  or sample size.

### Simulation Shortcuts
Making the experiment run faster reduce the variance, which improves many statistical measures of confidence
in estimates of means.

There are two main categories of shortcuts: 
1. Trial overloading: rewrite the test program to output cost measurements for multiple design points
   in one trial. This technique creates correlation between the instances of test.
   Whether or not correlation makes a difference to the data analysis depends on what analysis technique
   is used. Lack of independence is problematic for some classic methods of inferential statistics (such
   as hypothesis testing, regression analysis, and calculations of confidence intervals.
   Correlation and imbalance in data samples are less problematic for other categories of data analysis, especially
   exploratory and graphical methods, where few a priori assumptions are made about the data.
2. Modeling tricks: exploit information available in a laboratory context to simulate an algorithm more
   efficiently than can be done by direct implementation.

## Data Analysis
Information scientists tell us that data, alone, have no value or meaning. When organized and interpreted,
data become information, which is useful for answering factual questions.
Data analysis is a process of inspecting, summarizing, and interpreting a set of data to transform 
it into something useful: information is the immediate result, and knowledge the ultimate goal.

Data analysis define some categories of data: 
1. Categorical data: qualitative data.
2. Ordinal data: can be ranked but has no scale
3. Interval data: can be represented on a scale with no natual zero point, so ratios are not meaningful.
   One common example is the temperature measured in scales like celcius or farenheith.
4. Ratio data: it is a numerical data with a natural zero point, so ratios and other arithmetic
   transformations have meaning. This category has the following subcategories:
   1. Counts and amounts: are accumulated totals of discrete and continuous quantities, respectively.
      They are always positive and often bounded at the low end but not on the high end. 
   2. Ratios and proportions: both result from dividing one number by another. A proportion represents
      part of a total and ranges between 0 and 1, while ratio can be greater than 1. Both are always
      positive.
   3. Differences: represent the distance between pairs of numbers and can be positive or negative.

One important guideline is: you should report outcomes as ratio data whenever possible, but not at the
price of lost information.

### Univariate data
A univariate data sample is a set of scalar numbers that represent outcomes from an experiment.

In terms of descriptive statistics, location (a central tendency of the data) and dispersion (how much
spread there is away from the center) are very important together. Do not report one withouth the other.

Some examples of summaries for both location and dispersion:
- Location: mean, median
- Dispersion: variance, standard deviation, interquartile range

### Guidelines
- Apply logarithmic transformation, or more generally a power transformation, to impose symmetry in a
  skewed data sample.
   

# References
1. McGeoch CC. A Guide to Experimental Algorithmics. Cambridge University Press; 2012.
