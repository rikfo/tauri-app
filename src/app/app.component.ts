import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styles: [
    `
      .logo.angular:hover {
        filter: drop-shadow(0 0 2em #e32727);
      }
    `,
  ],
  standalone: true,
})
export class AppComponent {
  greetingMessage = "";
  message = "";

  greet(name: string): void {
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }

  submit(num: string, fact: string): void {
    invoke<string>("save", { num, fact }).then((text) => {
      this.message = text;
    });
  }
}
