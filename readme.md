# metapacker

a tool for _managing_ minecraft modpacks

## why <sup>or "rationale"</sup>

you may be asking: "why would i use metapacker if [packwiz](https://github.com/packwiz/packwiz) is a thing?"

to put it shortly, packwiz is a great tool for _creating_ and _maintaining_ modpacks.
it's second to none, really. is in realizing the greatness of packwiz that one realizes its pitfalls.

it's a simple format that's relatively easy to implement and parse, based on a markup language thats humanly readable.
one can learn how packwiz work by solely reading the `.pw.toml` files. never had i needed to read its source to figure
out what it was doing.

packwiz is horrible at _deriving_ new modpacks from already existing modpacks. nor does it ever intend to do that,
and if it did its likely that'd be to the detriment of what makes it great:
how fast it is and its simplicity.

packwiz isn't expandable because it doesn't need to be. it's targeted at modpack creators as a bundler of
mods, assets and configs in a way that's platform-agnostic ("platform" in this case being both the loader,
and most importantly the _distribution website_).

end users shouldn't have to know that packwiz its used because it's transparent to them, all they end up with
is the transformation of said pack as either a `.mrpack` or whatever overwolf is doing on their end.

## `#[derive]` me some modpacks

metapacker attempts to solve a very nieche feature that may only happen with a small subset of modpack creators or
server owners. modpack _derivation_.

making a modpack from scratch using metapacker is certainly possible, but that's not the intent of this project at all.

### the problem

i have a server and i need to tack on some other mods, some of which are exclusive to my server (most of the time being
exclusive server-side), remove some mods that were causing trouble on prod till we either figure out a fix and submit it
to the authors (if the mod is open-source) or fix it trough "less friendly" ways.

if the modpack we plan on playing uses packwiz (and its publically available), we have the option to:

1. clone the repo
2. apply our changes (which will be pegged to the current HEAD of the pack)
3. repack it
4. redistribute it somehow

any updates to the underlying modpack have the possibility of breaking these
changes, and that's expected. the moment i chose this path i willingly take
solace in the eventual git merge conflicts that **will** happen when i try to
update the pack to a latest version.

### the other problem

vanilla-compatible servers know this one way too well.

we have, for optimization modpacks on modrinth:

- simply optimized
- fabulously optimized
- sodium plus
- adrenaline / additive (devin's packs)

we also want to add certain qol features and integrate them with the server as good as we can.

so we take one of these four and tack a minimap mod here, masa's mods there and simple voice chat.

after some time of players keeping suggesting stuff they find interesting, you end up with a collection
of 100ish mods, and something that should be easy to manage now it's a mess of uncountable merge conflicts
and hard to debug mod interoperability issues. <sub> turns out the cute quilt loading screen mod has some glitches
with intel gpus, but only on fridays when the heap is 66% full, and that makes the game crash because of some
other mod (and that only happens on windows)</sub>

### the proposed solution

what if we could do *this* instead
```
    from: modrinth/modpacks:sop
    using: resolver/mrpack
    
    modify:
        add modrinth/mod:simplevoicechat
        # choice for minimaps, required.
        add choice [
            modrinth/mod:journeymap
            modrinth/mod:xaeros-minimap
            modrinth/mod:travellers-map
        ]
        # adds a dependency mod if any of the
        # dependents are added
        lazy add dep {
            curseforge/mod:malilib
        } for [
            curseforge/mod:litematica
            curseforge/mod:minihud
            curseforge/mod:tweakeroo
            curseforge/mod:item-scroller
        ]
        
        # displays to the user a configurable section
        # of masady's mods.
        section "Masa's Mods" {
            # marks all of these mods as optional
            maybe {
                # allows any/none mods to added 
                any add [ 
                    curseforge/mod:litematica
                    curseforge/mod:minihud
                    curseforge/mod:tweakeroo
                    curseforge/mod:item-scroller
                ]
            }
        }
```

> **Note**
> this is just one possible interface with the metapacker standard.

> **Important**
> metapacker itself is just a *single json file* representing rules that are to be applied to a given
modpack.


those rules currently are: `add`, `remove`, `choice` and `swap`. 