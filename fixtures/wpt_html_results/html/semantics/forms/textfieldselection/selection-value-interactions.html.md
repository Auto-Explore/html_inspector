# html/semantics/forms/textfieldselection/selection-value-interactions.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/textfieldselection/selection-value-interactions.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title></title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<div id=target></div>
<script>
  var target = document.getElementById("target");
  var sometext = "something";
  var shorttext = "abc";
  var elemData = [
    {
      desc: "textarea not in body",
      factory: () => document.createElement("textarea"),
    },
    {
      desc: "input not in body",
      factory: () => document.createElement("input"),
    },
    {
      desc: "textarea in body",
      factory: () => document.body.appendChild(document.createElement("textarea")),
    },
    {
      desc: "input in body",
      factory: () => document.body.appendChild(document.createElement("input")),
    },
    {
      desc: "textarea in body with parsed default value",
      factory: () => {
        target.innerHTML = "<textarea>abcdefghij</textarea>"
        return target.querySelector("textarea");
      },
    },
    {
      desc: "input in body with parsed default value",
      factory: () => {
        target.innerHTML = "<input value='abcdefghij'>"
        return target.querySelector("input");
      },
    },
    {
      desc: "focused textarea",
      factory: () => {
        var t = document.body.appendChild(document.createElement("textarea"));
        t.focus();
        return t;
      },
    },
    {
      desc: "focused input",
      factory: () => {
        var i = document.body.appendChild(document.createElement("input"));
        i.focus();
        return i;
      },
    },
    {
      desc: "focused then blurred textarea",
      factory: () => {
        var t = document.body.appendChild(document.createElement("textarea"));
        t.focus();
        t.blur();
        return t;
      },
    },
    {
      desc: "focused then blurred input",
      factory: () => {
        var i = document.body.appendChild(document.createElement("input"));
        i.focus();
        i.blur()
        return i;
      },
    },
  ];

for (var data of elemData) {
  test(function() {
    var el = data.factory();
    this.add_cleanup(() => el.remove());
    el.defaultValue = sometext;
    assert_true(sometext.length > 8,
                "sometext too short, test won't work right");
    el.selectionStart = 4;
    el.selectionEnd = 6;
    el.setRangeText("xyz");
    el.defaultValue = "set range text";
    assert_equals(el.value, sometext.slice(0, 4) + "xyz" + sometext.slice(6),
                  "Calling setRangeText should set the value dirty flag");
  }, `value dirty flag behavior after setRangeText on ${data.desc}`);
}

for (var tag of ['input', 'textarea']) {
  test(function() {
    var el = document.createElement(tag);
    document.body.appendChild(el);
    this.add_cleanup(() => el.remove());

    for (let val of ["", "foo", "foobar"]) {
      el.value = val;
      assert_equals(el.selectionStart, val.length,
                   "element.selectionStart should be value.length");
      assert_equals(el.selectionEnd, val.length,
                    "element.selectionEnd should be value.length");
    }
  }, `selection is collapsed to the end after changing values on ${tag}`);

  test(function() {
    var el = document.createElement(tag);
    document.body.appendChild(el);
    this.add_cleanup(() => el.remove());

    el.value = "foobar"
    el.selectionStart = 2
    el.selectionEnd = 4
    el.value = "foobar"

    assert_equals(el.selectionStart, 2,
                  "element.selectionStart should be unchanged");
    assert_equals(el.selectionEnd, 4,
                  "element.selectionEnd should be unchanged");
  }, `selection is not collapsed to the end when value is set to its existing value on ${tag}`);
}

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 44,
        "byte_start": 37,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/forms/textfieldselection/selection-value-interactions.html"
}
```
