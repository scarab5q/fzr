# WORK IN PROGRESS NOT CURRENTLY WORKING AT ALL

# FZR - "fuzzer" - A modular/plugin based fuzzy searcher

## Introduction

    I personally have found that all fuzzy finders annoy me somewhat. Don't
    get me wrong fzf, skim, pick, selecta, [fzy](https://github.com/jhawthorn/fzy),
    etc ... are all wonderful tools but I keep finding myself switching between them
    or wishing that I was using a different one or that one of them had slightly
    differnt ergonomics. Suffice to say they each annoy in their own special way.
    fzy has my favourite search algorithm but waits for all the input to go in
    before I am able to start fuzzy searching for what im looking for making it feel
    slightly for search all of my files for large systems. I also see that locate
    has me covered there in that it already has my whole file system mapped for even
    faster searching but doesn't have a fuzzy finder associated with it. You may
    want to use a fuzzy finder to filter out some sellections using regex but maybe
    the fuzzy finder with the particular search algorithm that you like doesnt
    support regex filtering. Maybe further into the future there might be some
    revolutionary algorithm that would work amazingly but hasn't been implemented
    in your favourite finder. My other issue is that as it stands now (at least
    with the fuzzy finders I have used) for a given situation you either have to
    write out an incredibly long terminal command to get what you want for a given
    situation or you have to write out an incredibly long command or set a bunch of
    enviroment variables. This I dislike because I just don't feel it is as flexible
    or usable as you would like.

    So after trying all of these fuzzy finders I decided that what really needs to
    be made is a fuzzy finder that can be configured to **EXACTLY** how you would
    like it for any given situation. Sort of like how your favourite text editor
    works, using modular components so that you can avoid writing out long terminal
    commands or setting many enviroment variables. In the future I hope to get a
    plugin ecosystem that will lazily load based on what you want you want and
    when.

## How this all works
	every finder has the following components:
		- search input  
        
		- *preprocessing* (optional) 
         values that have been pre-assigned to the search items to improve search speed_
		
        - query input
		
        - matching
		
        - output - what needs to be done with the selected item/s

to be good unix citizens it makes more sense to make sure each one of these
components is as good as they can possibly be before bringing them all together
to get what you want

## Installation - **TODO**
## Configuration - **TODO**





