# html/semantics/interactive-elements/the-summary-element/interactive-content.html

Counts:
- errors: 3
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-summary-element/interactive-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>summary element: interactive content</title>
<link rel="author" title="Michael[tm] Smith" href="mailto:mike@w3.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-summary-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<details>
  <summary id=summary>
  <a id=a href="#">anchor element</a>
  <svg style="width: 160px; height: 100px" viewBox="0 0 100 100">
    <a href="#" id="svg_a"><text id="svg_text" x="50" y="90" text-anchor="middle">SVG anchor element</text></a>
  </svg>
  <svg style="width: 100px; height: 200px" viewBox="0 0 100 100">
    <foreignObject x="0" y="60" width="100" height="200" text-anchor="middle">
      <a xmlns="http://www.w3.org/1999/xhtml" href="#" id="svg_foreignObject_a">SVG foreignObject with HTML anchor element</a>
    </foreignObject>
  </svg>
  <audio id="audio" controls src=/media/sound_5.mp3></audio>
  <button id=button>button element</button>
  <embed id=embed src="/images/blue.png" height="100" width="100">
  <iframe id=iframe srcdoc="iframe element"></iframe>
  <img id=img_usemap usemap src=/media/poster.png></img>
  <img id=img src=/media/poster.png></img>
  <input type="text" value="input@type=text" id="input_text">
  <input type="search" value="input@type=search" id="input_search">
  <input type="tel" value="input@type=tel" id="input_tel">
  <input type="url" value="input@type=url" id="input_url">
  <input type="email" value="input@type=email" id="input_email">
  <input type="password" value="input@type=password" id="input_password">
  <input type="button" value="input@type=button" id="input_button">
  <input type="reset" id="input_reset">
  <input type="submit" id="input_submit">
  <input type="date" value="input@type=date" id="input_date">
  <input type="month" value="input@type=month" id="input_month">
  <input type="week" value="input@type=week" id="input_week">
  <input type="time" id="input_time">
  <input type="datetime-local" id="input_datetime-local">
  <input type="color" id="input_color">
  <input type="number" value="1337" id="input_number">
  <input type="range" id="input_range">
  <input type="checkbox" id="input_checkbox">
  <input type="radio" id="input_radio" disabled>
  <input type="file" id="input_file">
  <input type="image" id="input_image" src=/media/poster.png>
  <label id=label style="display: block">label element</label>
  <textarea value="textarea" id="textarea">textarea element</textarea>
  <video id="video" controls>
    <source src="/media/test-1s.mp4" type="video/mp4">
    <source src="/media/test-1s.webm" type="video/webm">
  </video>
  <div id="non-interactive">This is clickable summary text</div>
  </summary>
</details>

<script>
const details = document.querySelector("details");

promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("non-interactive"));
  assert_true(details.open)
}, "Clicking on non-interactive child of a <summary> opens its <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("a"));
  assert_false(details.open)
}, "Clicking an <a> link doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("svg_a"));
  assert_false(details.open)
}, "Clicking an SVG <a> link doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("svg_foreignObject_a"));
  assert_false(details.open)
}, "Clicking an HTML <a> link in an SVG <foreignObject> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("audio"));
  assert_false(details.open)
}, "Clicking an <audio> element doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("button"));
  assert_false(details.open)
}, "Clicking a <button> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("embed"));
  assert_false(details.open)
}, "Clicking the content of an <embed> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("iframe"));
  assert_false(details.open)
}, "Clicking in an <iframe> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("img_usemap"));
  assert_false(details.open)
}, "Clicking an <img> with a 'usemap' attribute doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("img"));
  assert_true(details.open)
}, "Clicking an <img> without a 'usemap' attribute opens <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_button"));
  assert_false(details.open)
}, "Clicking an <input type=button> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_reset"));
  assert_false(details.open)
}, "Clicking an <input type=reset> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_submit"));
  assert_false(details.open)
}, "Clicking an <input type=submit> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_text"));
  assert_false(details.open)
}, "Clicking an <input type=text> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_search"));
  assert_false(details.open)
}, "Clicking an <input type=search> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_tel"));
  assert_false(details.open)
}, "Clicking an <input type=tel> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_url"));
  assert_false(details.open)
}, "Clicking an <input type=url> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_email"));
  assert_false(details.open)
}, "Clicking an <input type=email> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_password"));
  assert_false(details.open)
}, "Clicking an <input type=password> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_date"));
  assert_false(details.open)
}, "Clicking an <input type=date> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_month"));
  assert_false(details.open)
}, "Clicking an <input type=month> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_week"));
  assert_false(details.open)
}, "Clicking an <input type=week> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_time"));
  assert_false(details.open)
}, "Clicking an <input type=time> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_datetime-local"));
  assert_false(details.open)
}, "Clicking an <input type=datetime-local> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_number"));
  assert_false(details.open)
}, "Clicking an <input type=number> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_range"));
  assert_false(details.open)
}, "Clicking an <input type=range> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_color"));
  assert_false(details.open)
}, "Clicking an <input type=color> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_checkbox"));
  assert_false(details.open)
}, "Clicking an <input type=checkbox> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_radio"));
  assert_false(details.open)
}, "Clicking an <input type=radio> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_file"));
  assert_false(details.open)
}, "Clicking an <input type=file> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("input_image"));
  assert_false(details.open)
}, "Clicking an <input type=image> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("label"));
  assert_false(details.open)
}, "Clicking a <label> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("textarea"));
  assert_false(details.open)
}, "Clicking in a <textarea> doesn't open <details>");
promise_test(async () => {
  details.open = false;
  await test_driver.click(document.getElementById("video"));
  assert_false(details.open)
}, "Clicking a <video> doesn't open <details>");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.usemap.bad_value",
      "message": "Bad value “” for attribute “usemap” on element “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 1321,
        "byte_start": 1273,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1321,
        "byte_start": 1273,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 1327,
        "byte_start": 1321,
        "col": 51,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1364,
        "byte_start": 1330,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 1370,
        "byte_start": 1364,
        "col": 37,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “input@type=url” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1618,
        "byte_start": 1562,
        "col": 3,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.input.date.invalid",
      "message": "Bad value “input@type=date” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1969,
        "byte_start": 1910,
        "col": 3,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.input.month.invalid",
      "message": "Bad value “input@type=month” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2034,
        "byte_start": 1972,
        "col": 3,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.input.week.invalid",
      "message": "Bad value “input@type=week” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2096,
        "byte_start": 2037,
        "col": 3,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2522,
        "byte_start": 2463,
        "col": 3,
        "line": 48
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
  "source_name": "html/semantics/interactive-elements/the-summary-element/interactive-content.html"
}
```
