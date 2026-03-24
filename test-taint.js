// test-taint.js
function getUser(req, res) {
    // 1. Untrusted user input is assigned to 'userId'
    const userId = req.query.id;
    
    // A perfectly safe variable
    const safeQuery = "SELECT * FROM users WHERE id = 1";

    // 2. Safe query executes (Should NOT be flagged)
    db.query(safeQuery);

    // 3. Tainted variable executes! (SQL INJECTION - SHOULD BE FLAGGED)
    db.query(userId);
}