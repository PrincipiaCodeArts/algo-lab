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
    - Full factorial designs maximize the information gained from one experiment
2. The horse race: Looks for winners and losers in the space of implementation
ideas. Ex. What is the best implementation for some algorithm?
3. Fitting functions: we know the form of some cost function and we want to 
determine its constant values.
4. Modeling: we want to find the correct function family to describe a given 
cost.




# References
1. McGeoch CC. A Guide to Experimental Algorithmics. Cambridge University Press; 2012.
