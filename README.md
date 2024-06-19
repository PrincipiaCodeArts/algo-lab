# algo-lab
Laboratory for algorithms. Use experimental approach to study algorithms. Implementations in rust.

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

# References
1. McGeoch CC. A Guide to Experimental Algorithmics. Cambridge University Press; 2012.