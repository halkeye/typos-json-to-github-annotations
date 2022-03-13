pipeline {
  agent {
    docker { image 'rust:latest' }
  }
  stages {
    stage('Build') {
      steps {
        sh "cargo build"
      }
    }

    stage('Test') {
      steps {
        sh "cargo test"
      }
    }

    stage('Clippy') {
      steps {
        sh "cargo clippy --all"
      }
      post {
        always {
          recordIssues tool: cargo()
        }
      }
    }

    stage('Rustfmt') {
      steps {
        sh "cargo fmt --all -- check"
      }
      post {
        always {
          recordIssues tool: cargo()
        }
      }
    }

    stage('Doc') {
      steps {
        // Not sure what to do with this yet
        sh "cargo doc"
      }
    }
  }
}
