# steament

## Game Questionnaire Style Comment Generator

Generate a game comment by choosing options.

Inspired by game comments from Steam.

[![asciicast](https://asciinema.org/a/663655.svg)](https://asciinema.org/a/663655)

## Install

```bash
git clone https://github.com/AllenWu233/steament
cd steament
cargo install --path .
```

## Uninstall

```bash
cargo uninstall steament
```

## Usage

```bash
steament

```

## Survey Template Syntax

### Default

Default template: [template.txt](template.txt)

### A Basic Section Example

```text
Title: <section1_title>
<option1>
<option2>
<...>
######

Title: <section2_title>
<option1>
<option2>
<...>
######
```

The first line of a section must begin with 'Title: '

The last line must be '######'

Empty lines doesn't matter

## Survey Example

```text
empty_checkbox: "☐"
selected_checkbox: "✓"

=======大小=======
☐ 百兆小游戏
☐ 1-5 G
☐ 5-10 G
☐ 10-20 G
☐ 20-50 G
☐ 50-100 G
☐ 100-200 G
☐ GMOD

=======图像=======
☐ 像素党秒天秒地！
☐ 无法和现实区分
☐ 赏心悦目
☐ 很美丽
☐ 一般
☐ 不怎么样
☐ 别盯着看太久
☐ 画图.exe

=======游戏体验=======
☐ 完美！！！
☐ 不错！
☐ 普普通通
☐ 不怎么样
☐ 盯着墙看都比玩它强
☐ 锻炼你的精神抗打击能力

=======声音效果=======
☐ 耳朵怀孕
☐ 感动人心
☐ 还不错
☐ 一般
☐ 不咋样
☐ 耳朵流产
☐ 音乐？什么音乐？

=======受众群体=======
☐ 儿童
☐ 青少年
☐ 成人
☐ 仅限成人

=======配置要求=======
☐ 能玩扫雷的电脑都能玩
☐ 入门配置
☐ 一般配置
☐ 进阶级配置
☐ 发烧级配置
☐ 神威•太湖之光

=======难度=======
☐ 点点点突突突炸炸炸
☐ 轻松愉快
☐ 得动动脑子
☐ 上手快精通难
☐ 烧脑

=======游戏寿命=======
☐ 玩完就退款
☐ 5-10小时
☐ 10-30小时
☐ 30-50小时
☐ 50-100小时
☐ GMOD
☐ 我觉得我能比它先去世

=======游戏粘性=======
☐ 可能不会完全通关(毫无新意的垃圾游戏)
☐ 通关一次(玩着还可以的游戏)-----不出BUG的情况下
☐ 通关2-5次(很不错的游戏)
☐ 通关5-10次(这游戏我他妈玩爆！)
☐ 通关10-20次(你出不出3？嗯？我就问你出不出三？)
☐ 通关20-50次(死忠粉专属选项)
☐ 通关∞次(？？？？)

=======操作=======
☐ 你家的宠物也能玩
☐ 简单
☐ 中等
☐ 困难
☐ 反人类
☐ 只狼

=======肝=======
☐ 护肝
☐ 如果你想玩出点花样
☐ 肝与不肝并不影响剧情
☐ 每关必肝
☐ 肝痛
☐ 必须24小时全天候地肝

=======肾=======
☐ 护肾
☐ 需要短暂休息了
☐ 需要休息一段时间了
☐ 每关必冲
☐ 肾痛
☐ 一时手冲一时爽，一直手冲一直爽

=======故事情节=======
☐ 故事?什么故事?
☐ 文字或语音剧情
☐ 平庸
☐ 还行
☐ 精彩
☐ 可以作为你的第二人生

=======平均一局游戏时间=======
☐ 办公室必备
☐ 短小精悍
☐ 中等
☐ 较长
☐ 卧槽！天亮了！

=======联机=======
☐ 孤独患者
☐ 你唯一的沙雕网友
☐ 2-5人
☐ 5-10人
☐ 10-30人
☐ 30-64人
☐ 行星边际2

=======语种=======
☐ 你信不信我会八国语言？
☐ 基础英文
☐ 可以打汉化MOD或补丁
☐ 自带汉化！

=======价格=======
☐ 免费游玩!
☐ 业界良心！
☐ 值这个价
☐ 如果你有点闲钱的话可以一试
☐ 不推荐原价入
☐ 信仰无价

=======游戏整体评分=======
☐ ？- ？分(暂时无法评定分数)
☐ 0-10分(垃圾游戏、差评)
☐ 10-30分(完完全全的不及格)
☐ 30-50分(总体不是很和谐，但是买来当笑话看应该不是什么大问题)
☐ 50-65分(勉强及格，算是个游戏)
☐ 65-80分(算是个不错的游戏了)
☐ 80-95分(无论从哪个角度来看都非常不错)
☐ 95-100分(这游戏牛逼！)
☐ 100-100+分(完全超出预定水准，无论从哪方面看都是精品)
☐ ∞分(死忠舔狗专属评分)

=======Bug=======
☐ 几乎没有
☐ 少量
☐ bug令人烦恼
☐ 育碧
☐ 模拟山羊
```

## TODO

- [ ] Steam text formatting support
- [ ] CLI options
- [ ] Custom template
