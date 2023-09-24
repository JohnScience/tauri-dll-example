# Example of using a DLL from a Tauri application

## The story

This was done by request of a guy who wanted to launch Rust the game with a launcher written in Rust programming language.

<img width="373" alt="Capture" src="https://github.com/JohnScience/tauri-dll-example/assets/16991108/0ac784a1-3949-47f6-b9d1-74e20f38f4fe">

His message on Telegram

---

<img width="490" alt="Translation" src="https://github.com/JohnScience/tauri-dll-example/assets/16991108/8be2d3ca-d329-4310-ac4f-2117f96c738f">

Translation.

---

After quick negotiations, I agreed to complete the task and finished it within 1h30m. I tested the example using my DLL and tailored the code for their use case.

At this point they needed to do the following:

* add their `UnityPlayer.dll` to `src-tauri/dlls`;
* remove `src-tauri/dlls/my_dll.dll` from there;
* ~change `tauri.bundle.resources` from `"resources": ["dlls/my_dll.dll"]` to `"resources": ["dlls/UnityPlayer.dll"]`~
* enjoy the results.

## More screenshots

![Screenshot (320)](https://github.com/JohnScience/tauri-dll-example/assets/16991108/93d06cc7-7a87-4e8d-b1dc-a4cc36869b7e)

![Screenshot (321)](https://github.com/JohnScience/tauri-dll-example/assets/16991108/59a3e614-88ca-4189-8231-32ff5b2c1d1d)

![Screenshot (322)](https://github.com/JohnScience/tauri-dll-example/assets/16991108/fa4a6942-2720-4372-98a6-7473dd1c4f7e)

![Screenshot (323)](https://github.com/JohnScience/tauri-dll-example/assets/16991108/b3f4960e-0f1b-4ecd-8748-69b939fdecfb)

![Screenshot (324)](https://github.com/JohnScience/tauri-dll-example/assets/16991108/c55edec8-b01c-4cc1-a1ea-f5fd0ec590c3)
