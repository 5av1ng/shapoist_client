# Shpoist Client

Note: the translation may be low quality (e.g.: with many grammar mistakes)

## About

A rhythm game, with abstract gameplay object.

### Playing

Shapoist separates Notes and Judge Fields. As long as you click in Judge Fields at the right time,  you able to get scores. Besides, Shapoist also provides an animation editor (vary basic so far). By combine these two together, you can recreate most of rhythm game's gameplay (theoretically). (Note: The gameplay which have "sliders" can't be recreated for now, due to Shapoist didn't provide such function)

### Score And Accuracy

Shapoist has two scoring methods and five level of judgement.

1. Immaculate Mode

    | level of judgement | Accuracy                                                     | Judge interval |
    | ------------------ | ------------------------------------------------------------ | -------------- |
    | Immaculate         | $80\% + (1 - \frac{(ClickTime - JudgeTime)}{50ms})\cdot 20\% $ | ±50 ms         |
    | Extra              | $ 70\%$                                                      | ±70 ms         |
    | Normal             | $50\%$                                                       | ±120 ms        |
    | Fade               | $0\%$                                                        | ±150 ms        |
    | Miss               | $0\%$                                                        | N/A            |

    score calculation：$(\frac{Acc \cdot JudgedNotes}{TotalNotes} \times 0.95 + \frac{MaxCombo}{TotalNotes} \times 0.05 )  \times 2000000$

2. S-Extra Mode

    | level of judgement | Accuracy | Judge interval |
    | ------------------ | -------- | -------------- |
    | S-Extra            | $100\%$  | ±50 ms         |
    | Extra              | $ 70\%$  | ±70 ms         |
    | Normal             | $50\%$   | ±120 ms        |
    | Fade               | $0\%$    | ±150 ms        |
    | Miss               | $0\%$    | N/A            |

    score calculation：$(\frac{Acc \cdot JudgedNotes}{TotalNotes} \times 0.95 + \frac{MaxCombo}{TotalNotes} \times 0.05 )  \times 1000000$

### Platform

Windows MacOS Linux （[For Android click here](https://github.com/5av1ng/adgk-shapoist)) were supported, theoretically. If you are MacOS or Linux User, you need compile by yourself.

## Developing

The project made with rust. For more details, see ReadMeEnUs in /src path.

**Warning**: Currently, Shapoist were badly coded.

## LICENSE

MIT

Default Source：

* Icon：[xicons](xicons.org/) MIT/Apache 2
* Audio：CC-BY-SA