# html/rendering/widgets/button-layout/anonymous-button-content-box.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/anonymous-button-content-box.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Anonymous button content box</title>
<link rel=match href=anonymous-button-content-box-ref.html>
<style>
body { margin: 0 }
button, input { width: 124px; height: 124px; background: papayawhip; border: 2px solid; margin: 0; padding: 0; float: left; font: inherit }
body > div { clear: left; }
button > div { width: 50px; height: 50px; border: solid }
.grid { display: grid }
.flex { display: flex }
.tall { height: 150px }
.wide { width: 150px }
</style>
<div>
 <input type=button value=input>
 <input type=button value="input grid" class=grid>
 <input type=button value="input flex" class=flex>
 <button>button</button>
</div>
<div>
 <button style="text-align: left">button left</button>
 <button><div>div</div></button>
 <button class=grid>grid</button>
 <button class=grid><div>grid</div></button>
</div>
<div>
 <button class=flex>flex</button>
 <button class=flex><div>flex</div></button>
 <button><div class=tall>tall</div></button>
 <button><div class=wide>wide</div></button>
</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 725,
        "byte_start": 720,
        "col": 10,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 803,
        "byte_start": 798,
        "col": 21,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 895,
        "byte_start": 890,
        "col": 21,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 940,
        "byte_start": 924,
        "col": 10,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 985,
        "byte_start": 969,
        "col": 10,
        "line": 30
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/button-layout/anonymous-button-content-box.html"
}
```
