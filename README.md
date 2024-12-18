# 🌟 Rust CLI To-Do List Manager 🌟

A **colorful**, feature-rich, and interactive Command-Line Interface (**CLI**) to-do list manager written in **Rust**. Manage your tasks effortlessly with a sleek interface and persistent storage.

---

## ✨ Features

✅ **Add New Tasks**: Input a task description, priority (High, Medium, Low), and category (e.g., Work, Personal).  
✅ **View Tasks**: Beautifully formatted and color-coded task list.  
✅ **Edit Tasks**: Modify description, priority, or category seamlessly.  
✅ **Mark as Completed**: Keep track of completed tasks with a click.  
✅ **Remove Tasks**: Delete unwanted tasks in seconds.  
✅ **Sort Tasks**: Organize by priority or category for easy navigation.  
✅ **Persistent Storage**: Tasks saved in `tasks.json` for continuity.  

---

## 🎨 Color Legend

- **[x] Completed Tasks**: Displayed in **🟢 green**.
- **[ ] Pending Tasks**: Displayed in **🔴 red**.
- **Priority Levels**:
  - High: **🔴 Bold Red**
  - Medium: **🟡 Bold Yellow**
  - Low: **🟢 Bold Green**
- **Category Names**: Displayed in **🔵 Bold Blue**.

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: Install Rust from [Rust's official website](https://www.rust-lang.org/).

### Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/<your-username>/rust-cli-todo.git
   cd rust-cli-todo
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Run the application**:

   ```bash
   cargo run
   ```

---

## 💡 How It Works

1. **Launch the application.**
2. Select options from the interactive menu:
   - Add Task: Input task details.
   - Remove Task: Choose a task to delete.
   - Mark Completed: Mark tasks as done.
   - Edit Task: Update descriptions, priorities, or categories.
   - Sort Tasks: Organize tasks for better visibility.
   - Exit: Save tasks and quit.
3. Enjoy a visually appealing and user-friendly task management experience!

---

## 📂 Project Structure

- `src/main.rs`: Core functionality of the application.
- `tasks.json`: File for storing tasks persistently.

---

## 🎬 Example Output

```plaintext
To-Do List:
No.   ✔   Description          Priority   Category
1     [ ] Learn Rust          High       Programming
2     [x] Grocery Shopping    Medium     Personal
3     [ ] Submit Assignment   High       Work

Options: 1. Add Task  2. Remove Task  3. Mark Completed  4. Edit Task  5. Sort Tasks  6. Exit
Enter your choice:
```

---

## 🤝 Contributing

Contributions are welcome! Here’s how you can help:

1. **Fork the repository**.
2. **Create a branch** (`git checkout -b feature-name`).
3. **Commit your changes** (`git commit -m 'Add feature-name'`).
4. **Push to the branch** (`git push origin feature-name`).
5. **Submit a Pull Request**.

---

## 📜 License

This project is licensed under the **MIT License**. See the `LICENSE` file for details.

---

## 🌟 Acknowledgments

- 🙌 Special thanks to the Rust community for their incredible support and resources.
- 🚀 Inspired by the need for a lightweight and efficient task manager.

---

## 📞 Contact

For any questions or feedback, feel free to reach out:

- **GitHub**: [Your GitHub Profile](https://github.com/<your-username>)
- **Email**: your.email@example.com

---

🎉 **Enjoy managing your tasks efficiently with the Rust CLI To-Do List Manager!** 🎉
