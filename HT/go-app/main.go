package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

/* -------------------------------------------------------------------------- */
/*                            SERVIDOR BASICO EN GO                           */
/* -------------------------------------------------------------------------- */

/* ---------------------- Estructura de los estudiantes --------------------- */
type Student struct {
	Student   string `json:"student"`
	Age       int    `json:"age"`
	Faculty   string `json:"faculty"`
	Discipline int   `json:"discipline"`
}

/* ---------------------------------- POST ---------------------------------- */
func handler(w http.ResponseWriter, r *http.Request) {
	var students []Student
	err := json.NewDecoder(r.Body).Decode(&students)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	for _, student := range students {
		fmt.Fprintf(w, "- Student: %s, Age: %d, Faculty: %s, Discipline: %d\n\n", student.Student, student.Age, student.Faculty, student.Discipline)
	}
}
/* ---------------------------------- Main ---------------------------------- */
func main() {
	http.HandleFunc("/", handler)
	log.Println("Server started on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
