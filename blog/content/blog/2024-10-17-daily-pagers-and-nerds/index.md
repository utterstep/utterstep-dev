+++
authors = ["Vlad Stepanov"]
title = "2024-10-17: Pagers and Nerds"
description = "On why should you always have a nerd in your team."
[taxonomies]
tags = ["daily", "israel", "investigation"]
[extra]
toc = false
toc_inline = false
+++

Yesterday, Reuters published a piece on the
[Israeli plot to blast thousands of Hezbollah pagers](https://www.reuters.com/graphics/ISRAEL-PALESTINIANS/HEZBOLLAH-PAGERS/mopawkkwjpa/)
across Lebanon.

A quick recap: recently, at the beginning of September 2024, Israel
[executed](https://en.wikipedia.org/wiki/2024_Lebanon_pager_explosions) its plan targeting
top Hezbollah officials by exploding thousands of pagers in Lebanon. A month later, we are
beginning to get the details of the operation. Explosives were planted inside the batteries of
the pagers, sandwiched between two power cells, with non-metallic detonators to avoid detection.

<details>
<summary>Ethical Tangent</summary>

I, myself, have conflicted thoughts on that operation. On one hand, it's a very precise and
well-executed plan, with a lot of thought put into how to target only the enemy combatants.
On the other hand, with even as precise an attack as this one, there
[are still significant civilian casualties](https://www.theguardian.com/world/2024/sep/20/we-are-isolated-tired-scared-pager-attack-leaves-lebanon-in-shock)
including at least two children (9 and 12 years old) and two health workers.

And this attack sets a very dangerous precedent for the future – we don't want to expect our
civilian devices to blow up in our hands on the whim of a foreign power.

But unfortunately, this is the world we live in now.

</details>

It's an interesting read by itself, but what caught my eye was the part about the battery capacity.

<figure>
{{ image(url="pager-schema.png", alt="The schematic representation of the battery size relative to the whole pager. There are the markings on a battery: Lithium-ion Battery Pack; 7.4V; 300mAh; 2.22Wh. Below it, we can find a graph comparing the expected battery capacity (~8.7 Wh) to the actual one (2.22Wh).", full=true, no_hover=true ) }}
<figcaption>

Screenshot from the Reuters article. © [Reuters](https://www.reuters.com/graphics/ISRAEL-PALESTINIANS/HEZBOLLAH-PAGERS/mopawkkwjpa/)

</figcaption>
</figure>

As you can see above, the battery has **less than one-third** of the capacity expected for a battery of
this weight and size – `2.22Wh` vs the expected `~8.7Wh`.

The density of lithium-ion batteries is roughly the same as PETN explosives used. In the article, it
is mentioned that there was 6g of explosive in each 35g battery. That's roughly 17% of the weight.
Interestingly, this 17% wasted weight gave a 66% reduction in the battery capacity! Sure, some
space was also dedicated to the detonator, but it's still a sharp drop.

I think that it's due to all the wrapping and the casing that the battery has – having split the
battery in the two parts you need to have ~twice as much of the casing, which reduces the space
you have for the actual battery.

The article says that the pagers came through "serious procurement procedures" by Hezbollah –
how did they miss that detail?

<aside>

Quick tangent – I _hate_ ampere-hours as a unit because they make sense only in the context
of voltage, and the voltage is usually not specified and just assumed to be the standard `3.7V`.

</aside>

Sure, you don't hear about watt-hour values every day, but there is also a mAh value specified:
`300mAh`, and anyone who ever bought a power bank knows that the usual numbers are in the range
of thousands. Even accounting for the voltage, it means that in terms of the commonly used `3.7V`
the battery has `600mAh` capacity.

Also, `7.4V`? That's a weird voltage for a battery, and as far as I know – pagers usually operate
on lower voltages, like `3.7V`. Given that `7.4V` is twice the usual `3.7V`, it seems that Israeli
engineers used two cells in series, which could also trigger some alarms during the due diligence
process.

But, it seems like no one significantly nerdy was involved in the procurement process on
Hezbollah's side.
